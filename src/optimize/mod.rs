pub(crate) mod options;
mod job;
mod jpeg;
mod png;
pub use jpeg::Jpeg;
pub use png::Png;
pub(crate) use job::OptimizeJob;

use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

#[derive(Clone)]
pub struct OptimToken {
    pub id: u64,
    pub running: Arc<AtomicBool>,
    pub canceled: Arc<Mutex<HashSet<u64>>>,
}

impl OptimToken {
    /// 最適化が中止されたかどうかを返す
    /// * `return` - 最適化が中止されたかどうか
    pub fn is_canceled(&self) -> bool {
        !self.running.load(Ordering::Relaxed) || self.canceled.lock().unwrap().contains(&self.id)
    }
}
