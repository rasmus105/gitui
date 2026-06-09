use crate::git;

#[derive(Default)]
pub enum Loadable<T> {
    #[default]
    NotRequested,
    Loading,
    Loaded(T),
    Failed(String),
}

#[derive(Default)]
pub struct State {
    pub status: Loadable<git::Status>,
}
