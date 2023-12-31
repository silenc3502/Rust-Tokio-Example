use std::sync::{Arc, Mutex, Once};
use lazy_static::lazy_static;
use crate::thread_control::entity::thread_worker::ThreadWorker;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;

use crate::thread_control::repository::thread_worker_repository_impl::ThreadWorkerRepositoryImpl;
use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;

lazy_static! {
    static ref THREAD_SERVICE: Arc<Mutex<Option<ThreadWorkerServiceImpl>>> = Arc::new(Mutex::new(None));
    static ref INIT: Once = Once::new();
}

pub struct ThreadWorkerServiceImpl {
    repository: Arc<Mutex<Option<ThreadWorkerRepositoryImpl>>>,
}

impl ThreadWorkerServiceImpl {
    pub fn new(repository: Arc<Mutex<Option<ThreadWorkerRepositoryImpl>>>) -> Self {
        ThreadWorkerServiceImpl { repository }
    }

    pub fn get_instance() -> Arc<Mutex<Option<ThreadWorkerServiceImpl>>> {
        INIT.call_once(|| {
            let repository = ThreadWorkerRepositoryImpl::get_instance();
            let instance = Arc::new(Mutex::new(Some(ThreadWorkerServiceImpl::new(repository.clone()))));
            *THREAD_SERVICE.lock().unwrap() = Some(ThreadWorkerServiceImpl::new(repository));
        });
        THREAD_SERVICE.clone()
    }
}

impl ThreadWorkerServiceTrait for ThreadWorkerServiceImpl {
    fn create_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>) {
        let repository_guard = self.repository.lock().unwrap();

        if let Some(repository) = repository_guard.as_ref() {
            repository.save_thread_worker(name, custom_function);
        }
    }

    fn get_thread(&self, name: &str) -> Option<ThreadWorker> {
        let repository_guard = self.repository.lock().unwrap();

        repository_guard.as_ref().and_then(|repository| repository.find_by_name(name))
    }

    // fn start_worker(&self, name: &str) {
    //     let repository = self.repository.lock().unwrap();
    //     repository.as_ref().unwrap().start_worker_thread(name);
    // }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    #[test]
    fn test_singleton_behavior() {
        let instance1 = ThreadWorkerServiceImpl::get_instance();
        let instance2 = ThreadWorkerServiceImpl::get_instance();

        assert!(Arc::ptr_eq(&instance1, &instance2));

        instance1.lock().unwrap().as_ref().unwrap().create_thread("Alice", None);
        let retrieved_worker = instance2.lock().unwrap().as_ref().unwrap().get_thread("Alice");

        assert_eq!(retrieved_worker.map(|w| w.name().to_owned()), Some("Alice".to_owned()));
    }

    #[test]
    fn test_create_and_get_thread() {
        let instance = ThreadWorkerServiceImpl::get_instance();
        instance.lock().unwrap().as_ref().unwrap().create_thread("Bob", None);

        let retrieved_worker = instance.lock().unwrap().as_ref().unwrap().get_thread("Bob");
        assert_eq!(retrieved_worker.map(|w| w.name().to_owned()), Some("Bob".to_owned()));
    }

    #[test]
    fn test_get_nonexistent_thread() {
        let instance = ThreadWorkerServiceImpl::get_instance();
        let retrieved_worker = instance.lock().unwrap().as_ref().unwrap().get_thread("Nonexistent");

        assert_eq!(retrieved_worker, None);
    }

    // #[test]
    // fn test_start_worker_thread_with_custom_function() {
    //     let instance = ThreadWorkerServiceImpl::get_instance();
    //
    //     let custom_function_executed = Arc::new(Mutex::new(false));
    //     let custom_function_executed_clone = Arc::clone(&custom_function_executed);
    //     let custom_function = move || {
    //         println!("Custom function executed!");
    //         *custom_function_executed_clone.lock().unwrap() = true;
    //     };
    //     let worker_name = "Alice";
    //     instance
    //         .lock()
    //         .unwrap()
    //         .as_ref()
    //         .unwrap()
    //         .create_thread(worker_name, Some(Box::new(custom_function.clone())));
    //
    //     instance
    //         .lock()
    //         .unwrap()
    //         .as_ref()
    //         .unwrap()
    //         .start_worker(worker_name);
    //
    //     thread::sleep(std::time::Duration::from_secs(1));
    //
    //     let lock = custom_function_executed.lock().unwrap();
    //     assert_eq!(*lock, true);
    // }
}