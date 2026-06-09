use super::status;
use crate::git;
use crossbeam::channel::{Receiver, Sender, unbounded};
use enum_map::{Enum, EnumMap};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};
use strum::EnumDiscriminants;

// ====================================================================================================
// Public types
// ====================================================================================================

/// A git operation request, optionally carrying parameters.
/// The hash of the full value (type + params) is used to detect stale results:
/// if params change while a job is running, the result is silently discarded.
///
/// Adding a new operation: add a variant here and an arm to [`GitWorkerInner::run_op`].
/// `OpType` and its [`EnumMap`] slot are derived automatically.
#[derive(Clone, Hash, EnumDiscriminants)]
#[strum_discriminants(name(OpType))]
#[strum_discriminants(derive(Enum))]
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
    slots: EnumMap<OpType, JobSlot>,
}

#[derive(Default)]
struct JobSlot {
    pending: AtomicBool,
    inner: Mutex<SlotInner>,
}

#[derive(Default)]
struct SlotInner {
    queued: Option<GitOperation>,
    /// Hash of the most recently enqueued request. A completed job whose hash
    /// no longer matches has been superseded and its result is discarded.
    expected_hash: u64,
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
        let hash = op_hash(&op);
        let op_type = OpType::from(&op);
        {
            let mut inner = self.slots[op_type].inner.lock().unwrap();
            inner.queued = Some(op);
            inner.expected_hash = hash;
        }
        self.spawn_if_idle(op_type);
    }

    fn spawn_if_idle(self: &Arc<Self>, op_type: OpType) {
        let slot = &self.slots[op_type];
        if slot.pending.swap(true, Ordering::AcqRel) {
            return;
        }
        let queued = slot.inner.lock().unwrap().queued.take();
        let Some(op) = queued else {
            slot.pending.store(false, Ordering::Release);
            return;
        };
        let my_hash = op_hash(&op);
        let worker = Arc::clone(self);
        self.pool.spawn(move || worker.run(op_type, op, my_hash));
    }

    fn run(self: Arc<Self>, op_type: OpType, op: GitOperation, my_hash: u64) {
        let event = self.run_op(&op);
        let expected = self.slots[op_type].inner.lock().unwrap().expected_hash;
        if expected == my_hash {
            let _ = self.events.send(event);
        }
        self.slots[op_type].pending.store(false, Ordering::Release);
        self.spawn_if_idle(op_type);
    }

    fn run_op(&self, op: &GitOperation) -> GitEvent {
        match op {
            GitOperation::Status => match status::load(&self.repo_path) {
                Ok(s) => GitEvent::Status(s),
                Err(e) => GitEvent::Failed { operation: op.clone(), error: e.to_string() },
            },
        }
    }
}

// ====================================================================================================
// Helpers
// ====================================================================================================

fn op_hash(op: &GitOperation) -> u64 {
    let mut hasher = DefaultHasher::new();
    op.hash(&mut hasher);
    hasher.finish()
}
