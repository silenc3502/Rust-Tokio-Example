// use std::collections::HashMap;
// use std::sync::{Arc, RwLock};
// use std::thread::ThreadId;
// use lazy_static::lazy_static;
// use crate::thread_manage::entity::worker::Worker;
// use crate::thread_manage::repository::worker_repository::WorkerRepositoryTrait;
//
// lazy_static! {
//     static ref THREADS: Arc<RwLock<HashMap<ThreadId, Worker>>> = Arc::new(RwLock::new(HashMap::new()));
// }
//
// #[derive(Debug)]
// pub struct WorkerRepositoryImpl {
//     pub threads: Arc<RwLock<HashMap<ThreadId, Worker>>>,
// }
//
// impl PartialEq for WorkerRepositoryImpl {
//     fn eq(&self, other: &Self) -> bool {
//         Arc::ptr_eq(&self.threads, &other.threads)
//     }
// }
//
// unsafe impl Sync for WorkerRepositoryImpl {}
//
// impl WorkerRepositoryImpl {
//     pub fn new() -> Self {
//         WorkerRepositoryImpl {
//             threads: Arc::clone(&THREADS),
//         }
//     }
// }
//
// impl Clone for WorkerRepositoryImpl {
//     fn clone(&self) -> Self {
//         WorkerRepositoryImpl {
//             threads: Arc::clone(&THREADS),
//         }
//     }
// }
//
// impl WorkerRepositoryTrait for WorkerRepositoryImpl {
//     fn save_thread(&self, thread_id: ThreadId, name: &str, custom_function: Option<Box<dyn Fn()>>) {
//         let mut threads = self.threads.write().unwrap();
//         let worker = Worker::new(name, custom_function);
//         threads.insert(thread_id, worker);
//     }
//
//     fn generate_thread_id(&self) -> ThreadId {
//         std::thread::current().id()
//     }
//
//     fn get_thread(&self, thread_id: ThreadId) -> Option<Worker> {
//         let threads = self.threads.read().unwrap();
//         threads.get(&thread_id).cloned()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_singleton_behavior() {
//         let repository1 = WorkerRepositoryImpl::new();
//         let repository2 = repository1.clone();
//
//         let arc1 = Arc::clone(&repository1.threads);
//         let arc2 = Arc::clone(&repository2.threads);
//
//         assert!(Arc::ptr_eq(&arc1, &arc2));
//     }
//
//     #[test]
//     fn test_save_and_get_thread() {
//         let repository = WorkerRepositoryImpl::new();
//
//         let thread_id = repository.generate_thread_id();
//         repository.save_thread(thread_id, "John Doe", None);
//
//         let retrieved_worker = repository.get_thread(thread_id);
//
//         assert_eq!(
//             retrieved_worker,
//             Some(Worker::new("John Doe", None))
//         );
//     }
//
//     #[test]
//     fn test_save_thread_with_custom_function() {
//         let repository = WorkerRepositoryImpl::new();
//
//         let thread_id = repository.generate_thread_id();
//         let custom_function = || {
//             println!("Custom function executed!");
//         };
//
//         repository.save_thread(thread_id, "John Doe", Some(Box::new(custom_function)));
//
//         let retrieved_worker = repository.get_thread(thread_id);
//
//         assert_eq!(
//             retrieved_worker,
//             Some(Worker::new("John Doe", Some(Box::new(custom_function))))
//         );
//     }
// }

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::Once;
use lazy_static::lazy_static;
use crate::thread_manage::entity::worker::Worker;
use crate::thread_manage::repository::worker_repository::WorkerRepositoryTrait;

#[derive(Debug, Clone)]
pub struct WorkerRepositoryImpl {
    workers: Arc<Mutex<HashMap<String, Worker>>>,
}

lazy_static! {
    static ref WORKER_REPOSITORY: Arc<Mutex<Option<WorkerRepositoryImpl>>> = Arc::new(Mutex::new(None));
    static ref INIT: Once = Once::new();
}

fn initialize_repository() -> WorkerRepositoryImpl {
    WorkerRepositoryImpl {
        workers: Arc::new(Mutex::new(HashMap::new())),
    }
}

impl WorkerRepositoryImpl {
    pub fn new() -> Self {
        initialize_repository()
    }

    pub fn get_instance() -> Arc<Mutex<Option<WorkerRepositoryImpl>>> {
        INIT.call_once(|| {
            *WORKER_REPOSITORY.lock().unwrap() = Some(initialize_repository());
        });
        WORKER_REPOSITORY.clone()
    }
}

impl WorkerRepositoryTrait for WorkerRepositoryImpl {
    fn save_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>) {
        let mut workers = self.workers.lock().unwrap();
        let worker = Worker::new(name, custom_function);
        workers.insert(name.to_string(), worker);
    }

    fn get_thread(&self, name: &str) -> Option<Worker> {
        let workers = self.workers.lock().unwrap();
        workers.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_behavior() {
        let repository1 = WorkerRepositoryImpl::new();
        let repository2 = repository1.clone();

        assert!(Arc::ptr_eq(&repository1.workers, &repository2.workers));
    }

    #[test]
    fn test_save_thread() {
        let repository = WorkerRepositoryImpl::new();
        repository.save_thread("John Doe", None);

        let worker = repository.get_thread("John Doe").expect("Worker not found.");
        assert_eq!(worker.name(), "John Doe");
        assert!(worker.custom_function().is_none());
    }

    #[test]
    fn test_save_thread_with_custom_function() {
        let repository = WorkerRepositoryImpl::new();
        let custom_function = || {
            println!("Custom function executed!");
        };

        repository.save_thread("John Doe", Some(Box::new(custom_function)));

        let worker = repository.get_thread("John Doe").expect("Worker not found.");
        assert_eq!(worker.name(), "John Doe");
        assert!(worker.custom_function().is_some());
    }

    #[test]
    fn test_get_thread_nonexistent() {
        let repository = WorkerRepositoryImpl::new();

        let worker = repository.get_thread("Nonexistent");
        assert!(worker.is_none());
    }

    #[test]
    fn test_get_thread_existing() {
        let repository = WorkerRepositoryImpl::new();
        repository.save_thread("Jane Doe", None);

        let worker = repository.get_thread("Jane Doe").expect("Worker not found.");
        assert_eq!(worker.name(), "Jane Doe");
    }

    // #[test]
    // fn test_save_and_get_thread() {
    //     let repository = WorkerRepositoryImpl::new();
    //
    //     let thread_id = repository.generate_thread_id();
    //     repository.save_thread(thread_id, "John Doe", None).unwrap();
    //
    //     let retrieved_worker = repository.get_thread(thread_id);
    //
    //     assert_eq!(
    //         retrieved_worker,
    //         Some(Worker::new("John Doe", None))
    //     );
    // }
    //
    // #[test]
    // fn test_save_thread_with_custom_function() {
    //     let repository = WorkerRepositoryImpl::new();
    //
    //     let thread_id = repository.generate_thread_id();
    //     let custom_function = || {
    //         println!("Custom function executed!");
    //     };
    //
    //     repository.save_thread(thread_id, "John Doe", Some(Box::new(custom_function))).unwrap();
    //
    //     let retrieved_worker = repository.get_thread(thread_id);
    //
    //     assert_eq!(
    //         retrieved_worker,
    //         Some(Worker::new("John Doe", Some(Box::new(custom_function))))
    //     );
    // }
}


