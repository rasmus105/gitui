use std::{fmt, path::PathBuf};

#[derive(Debug, Default)]
pub struct RepoState {
    pub file_changes: Vec<FileChange>,
}

#[derive(Debug)]
pub struct FileChange {
    pub path: PathBuf,
    pub old_path: Option<PathBuf>,
    pub index_status: Option<FileStatus>,
    pub worktree_status: Option<FileStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    TypeChanged,
    Untracked,
    Conflicted,
}

impl fmt::Display for FileStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileStatus::Added => f.write_str("added"),
            FileStatus::Modified => f.write_str("modified"),
            FileStatus::Deleted => f.write_str("deleted"),
            FileStatus::Renamed => f.write_str("renamed"),
            FileStatus::Copied => f.write_str("copied"),
            FileStatus::TypeChanged => f.write_str("type changed"),
            FileStatus::Untracked => f.write_str("untracked"),
            FileStatus::Conflicted => f.write_str("conflicted"),
        }
    }
}
