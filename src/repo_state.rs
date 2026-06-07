#[derive(Debug, Default)]
pub struct RepoState {
    pub changed_files: Vec<ChangedFile>,
}

#[derive(Debug)]
pub struct ChangedFile {
    pub path: String,
}
