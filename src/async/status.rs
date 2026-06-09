use crate::git;
use gix::bstr::{BStr, ByteSlice};
use gix::status::index_worktree;
use std::{collections::BTreeMap, path::PathBuf};

pub(super) fn load(repo_path: &PathBuf) -> anyhow::Result<git::Status> {
    let repo = gix::open(repo_path)?;
    let index = repo.index_or_empty()?;

    let mut changes = BTreeMap::<PathBuf, git::File>::new();
    let status = repo
        .status(gix::progress::Discard)?
        .index(index.clone().into())
        .untracked_files(gix::status::UntrackedFiles::Collapsed)
        .into_iter(Vec::new())?;

    for item in status {
        let item = item?;
        match item {
            gix::status::Item::IndexWorktree(item) => apply_worktree_change(&mut changes, item),
            gix::status::Item::TreeIndex(change) => apply_index_change(&mut changes, change),
        }
    }

    Ok(git::Status {
        files_changed: changes.into_values().collect(),
    })
}

fn apply_worktree_change(changes: &mut BTreeMap<PathBuf, git::File>, item: index_worktree::Item) {
    let Some(status) = item.summary().and_then(worktree_status) else {
        return;
    };

    let path = path_buf(item.rela_path());
    let old_path = match &item {
        index_worktree::Item::Rewrite { source, .. } => Some(path_buf(source.rela_path())),
        _ => None,
    };

    let change = changes.entry(path.clone()).or_insert_with(|| git::File {
        path,
        old_path: None,
        index_status: None,
        worktree_status: None,
    });

    change.worktree_status = Some(status);
    change.old_path = change.old_path.take().or(old_path);
}

fn apply_index_change(
    changes: &mut BTreeMap<PathBuf, git::File>,
    change: gix::diff::index::Change,
) {
    let (path, old_path, status) = match change {
        gix::diff::index::Change::Addition { location, .. } => {
            (path_buf(location.as_ref()), None, git::FileStatus::Added)
        }
        gix::diff::index::Change::Deletion { location, .. } => {
            (path_buf(location.as_ref()), None, git::FileStatus::Deleted)
        }
        gix::diff::index::Change::Modification { location, .. } => {
            (path_buf(location.as_ref()), None, git::FileStatus::Modified)
        }
        gix::diff::index::Change::Rewrite {
            source_location,
            location,
            copy,
            ..
        } => (
            path_buf(location.as_ref()),
            Some(path_buf(source_location.as_ref())),
            if copy {
                git::FileStatus::Copied
            } else {
                git::FileStatus::Renamed
            },
        ),
    };

    let change = changes.entry(path.clone()).or_insert_with(|| git::File {
        path,
        old_path: None,
        index_status: None,
        worktree_status: None,
    });

    change.index_status = Some(status);
    change.old_path = change.old_path.take().or(old_path);
}

fn worktree_status(summary: index_worktree::iter::Summary) -> Option<git::FileStatus> {
    match summary {
        index_worktree::iter::Summary::Removed => Some(git::FileStatus::Deleted),
        index_worktree::iter::Summary::Added => Some(git::FileStatus::Untracked),
        index_worktree::iter::Summary::Modified => Some(git::FileStatus::Modified),
        index_worktree::iter::Summary::TypeChange => Some(git::FileStatus::TypeChanged),
        index_worktree::iter::Summary::Renamed => Some(git::FileStatus::Renamed),
        index_worktree::iter::Summary::Copied => Some(git::FileStatus::Copied),
        index_worktree::iter::Summary::IntentToAdd => Some(git::FileStatus::Added),
        index_worktree::iter::Summary::Conflict => Some(git::FileStatus::Conflicted),
    }
}

fn path_buf(path: &BStr) -> PathBuf {
    PathBuf::from(path.to_str_lossy().into_owned())
}
