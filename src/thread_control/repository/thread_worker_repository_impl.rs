use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use lazy_static::lazy_static;
use crate::thread_control::entity::thread_worker::ThreadWorker;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;
use crate::thread_manage_legacy::entity::worker::Worker;

#[derive(Debug, Clone)]
pub struct ThreadWorkerRepositoryImpl {
    thread_worker_list: Arc<Mutex<HashMap<String, ThreadWorker>>>,
}

lazy_static! {
    static ref WORKER_REPOSITORY: Arc<Mutex<Option<ThreadWorkerRepositoryImpl>>> = Arc::new(Mutex::new(None));
    static ref INIT: Once = Once::new();
}

fn initialize_repository() -> ThreadWorkerRepositoryImpl {
    ThreadWorkerRepositoryImpl {
        thread_worker_list: Arc::new(Mutex::new(HashMap::new())),
    }
}

impl ThreadWorkerRepositoryImpl {
    pub fn new() -> Self {
        initialize_repository()
    }

    pub fn get_instance() -> Arc<Mutex<Option<ThreadWorkerRepositoryImpl>>> {
        INIT.call_once(|| {
            *WORKER_REPOSITORY.lock().unwrap() = Some(initialize_repository());
        });
        WORKER_REPOSITORY.clone()
    }
}

impl ThreadWorkerRepositoryTrait for ThreadWorkerRepositoryImpl {
    fn save_thread_worker(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>) {
        let mut thread_worker_list = self.thread_worker_list.lock().unwrap();
        let thread_worker = ThreadWorker::new(name, custom_function);
        thread_worker_list.insert(name.to_string(), thread_worker);
    }

    fn find_by_name(&self, name: &str) -> Option<ThreadWorker> {
        let thread_worker_list = self.thread_worker_list.lock().unwrap();
        thread_worker_list.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Weak;
    use std::thread;
    use crate::thread_manage_legacy::entity::worker::Worker;
    use crate::thread_manage_legacy::repository::worker_repository_impl::WorkerRepositoryImpl;
    use super::*;

    #[test]
    fn test_singleton_behavior() {
        let repository1 = ThreadWorkerRepositoryImpl::new();
        let repository2 = repository1.clone();

        assert!(Arc::ptr_eq(&repository1.thread_worker_list, &repository2.thread_worker_list));
    }

    #[test]
    fn test_save_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::new();
        repository.save_thread_worker("John Doe", None);

        let thread_worker = repository.find_by_name("John Doe").expect("Worker not found.");
        assert_eq!(thread_worker.name(), "John Doe");
        assert!(thread_worker.cloned_custom_function().is_none()); // 수정된 부분
    }

    #[test]
    fn test_save_thread_with_custom_function() {
        let repository = ThreadWorkerRepositoryImpl::new();
        let custom_function = || {
            println!("Custom function executed!");
        };

        repository.save_thread_worker("John Doe", Some(Box::new(custom_function)));

        let thread_worker = repository.find_by_name("John Doe").expect("Worker not found.");
        assert_eq!(thread_worker.name(), "John Doe");
        assert!(thread_worker.cloned_custom_function().is_some());
    }

    #[test]
    fn test_get_thread_nonexistent() {
        let repository = ThreadWorkerRepositoryImpl::new();

        let thread_worker = repository.find_by_name("Nonexistent");
        assert!(thread_worker.is_none());
    }

    #[test]
    fn test_get_thread_existing() {
        let repository = ThreadWorkerRepositoryImpl::new();
        repository.save_thread_worker("Jane Doe", None);

        let thread_worker = repository.find_by_name("Jane Doe").expect("Worker not found.");
        assert_eq!(thread_worker.name(), "Jane Doe");
    }

    // #[test]
    // fn test_start_worker_thread_with_custom_function() {
    //     let custom_function_executed = Arc::new(Mutex::new(false));
    //     let custom_function_executed_clone = Arc::clone(&custom_function_executed);
    //     let custom_function = move || {
    //         println!("Custom function executed!");
    //         // Set the flag to true when the function is executed
    //         let mut lock = custom_function_executed_clone.lock().unwrap();
    //         *lock = true;
    //     };
    //     let worker = Worker::new("Alice", Some(Box::new(custom_function.clone())));
    //
    //     // Create a repository with the worker
    //     let repository = WorkerRepositoryImpl::new();
    //     repository.save_thread("Alice", Some(Box::new(custom_function)));
    //
    //     // Start the worker thread using the repository
    //     repository.start_worker_thread("Alice");
    //
    //     // Allow some time for the thread to execute
    //     thread::sleep(std::time::Duration::from_secs(1));
    //
    //     // Check if the custom function was executed
    //     let lock = custom_function_executed.lock().unwrap();
    //     assert_eq!(*lock, true);
    // }
    //
    // #[test]
    // fn test_start_worker_thread_without_custom_function() {
    //     // Create a worker without a custom function
    //     let worker = Worker::new("Bob", None);
    //
    //     // Create a repository with the worker
    //     let repository = WorkerRepositoryImpl::new();
    //     repository.save_thread("Bob", None);
    //
    //     // Start the worker thread using the repository
    //     repository.start_worker_thread("Bob");
    //
    //     // No custom function, so the thread won't do anything
    // }
}