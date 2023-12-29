use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread::ThreadId;
use lazy_static::lazy_static;
use crate::thread_manage::entity::worker::Worker;
use crate::thread_manage::repository::worker_repository::WorkerRepositoryTrait;

lazy_static! {
    static ref THREADS: Arc<RwLock<HashMap<ThreadId, Worker>>> = Arc::new(RwLock::new(HashMap::new()));
}

#[derive(Debug)]
pub struct WorkerRepositoryImpl {
    pub threads: Arc<RwLock<HashMap<ThreadId, Worker>>>,
}

impl PartialEq for WorkerRepositoryImpl {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.threads, &other.threads)
    }
}

unsafe impl Sync for WorkerRepositoryImpl {}

impl WorkerRepositoryImpl {
    pub fn new() -> Self {
        WorkerRepositoryImpl {
            threads: Arc::clone(&THREADS),
        }
    }
}

impl Clone for WorkerRepositoryImpl {
    fn clone(&self) -> Self {
        WorkerRepositoryImpl {
            threads: Arc::clone(&THREADS),
        }
    }
}

impl WorkerRepositoryTrait for WorkerRepositoryImpl {
    fn save_thread(&self, thread_id: ThreadId, name: &str) {
        let mut threads = self.threads.write().unwrap();
        let worker = Worker::new(name);
        threads.insert(thread_id, worker);
    }

    fn generate_thread_id(&self) -> ThreadId {
        std::thread::current().id()
    }

    fn get_thread(&self, thread_id: ThreadId) -> Option<Worker> {
        let threads = self.threads.read().unwrap();
        threads.get(&thread_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_behavior() {
        let repository1 = WorkerRepositoryImpl::new();
        let repository2 = repository1.clone();

        let arc1 = Arc::clone(&repository1.threads);
        let arc2 = Arc::clone(&repository2.threads);

        assert!(Arc::ptr_eq(&arc1, &arc2));
    }
}