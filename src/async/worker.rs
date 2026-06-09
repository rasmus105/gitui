use super::status;
use crate::git;
use crossbeam::channel::{Receiver, Sender, unbounded};
use enum_map::{Enum, EnumMap};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

// ====================================================================================================
// Public types
// ====================================================================================================

#[derive(Clone, Copy, Enum)]
pub enum GitOperation {
    Status,
}

pub enum GitEvent {
    Status(git::Status),
    Failed { operation: GitOperation, error: String },
}

pub struct GitWorker {
    events_rx: Receiver<GitEvent>,
    inner: Arc<GitWorkerInner>,
}

// ====================================================================================================
// Private types
// ====================================================================================================

struct GitWorkerInner {
    repo_path: PathBuf,
    events: Sender<GitEvent>,
    pool: Arc<ThreadPool>,
    slots: EnumMap<GitOperation, JobSlot>,
}

#[derive(Default)]
struct JobSlot {
    pending: AtomicBool,
    queued: AtomicBool,
}

// ====================================================================================================
// Public API
// ====================================================================================================

impl GitWorker {
    pub fn spawn(repo_path: PathBuf) -> anyhow::Result<Self> {
        let (event_tx, event_rx) = unbounded();
        let pool = Arc::new(
            ThreadPoolBuilder::new()
                .num_threads(0)
                .thread_name(|idx| format!("gitui-git-{idx}"))
                .build()?,
        );

        Ok(Self {
            events_rx: event_rx,
            inner: Arc::new(GitWorkerInner {
                repo_path,
                events: event_tx,
                pool,
                slots: EnumMap::default(),
            }),
        })
    }

    pub fn request(&self, op: GitOperation) {
        self.inner.enqueue(op);
    }

    pub fn drain(&self) -> impl Iterator<Item = GitEvent> + '_ {
        std::iter::from_fn(|| self.events_rx.try_recv().ok())
    }
}

// ====================================================================================================
// Scheduling
// ====================================================================================================

impl GitWorkerInner {
    fn enqueue(self: &Arc<Self>, op: GitOperation) {
        self.slots[op].queued.store(true, Ordering::Release);
        self.spawn_if_idle(op);
    }

    fn spawn_if_idle(self: &Arc<Self>, op: GitOperation) {
        let slot = &self.slots[op];
        if slot.pending.swap(true, Ordering::AcqRel) {
            return;
        }
        if !slot.queued.swap(false, Ordering::AcqRel) {
            slot.pending.store(false, Ordering::Release);
            return;
        }
        let inner = Arc::clone(self);
        self.pool.spawn(move || inner.run(op));
    }

    fn run(self: Arc<Self>, op: GitOperation) {
        let event = self.run_op(op);
        let _ = self.events.send(event);
        self.slots[op].pending.store(false, Ordering::Release);
        self.spawn_if_idle(op);
    }

    fn run_op(&self, op: GitOperation) -> GitEvent {
        match op {
            GitOperation::Status => match status::load(&self.repo_path) {
                Ok(s) => GitEvent::Status(s),
                Err(e) => GitEvent::Failed { operation: op, error: e.to_string() },
            },
        }
    }
}
