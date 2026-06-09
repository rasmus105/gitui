use std::path::PathBuf;

pub struct Status {
    pub files_changed: Vec<File>,
}

pub struct File {
    pub path: PathBuf,
    pub old_path: Option<PathBuf>,
    pub index_status: Option<FileStatus>,
    pub worktree_status: Option<FileStatus>,
}

#[derive(Clone, Copy)]
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
