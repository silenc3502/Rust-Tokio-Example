use std::sync::{Arc, Mutex, Once};
use lazy_static::lazy_static;
use crate::thread_manage_legacy::entity::worker::Worker;
use crate::thread_manage_legacy::repository::worker_repository::WorkerRepositoryTrait;
use crate::thread_manage_legacy::repository::worker_repository_impl::WorkerRepositoryImpl;
use crate::thread_manage_legacy::service::worker_service::WorkerServiceTrait;

lazy_static! {
    static ref THREAD_SERVICE: Arc<Mutex<Option<WorkerServiceImpl>>> = Arc::new(Mutex::new(None));
    static ref INIT: Once = Once::new();
}

// Implement the ThreadServiceImpl
pub struct WorkerServiceImpl {
    repository: Arc<Mutex<Option<WorkerRepositoryImpl>>>,
}

impl WorkerServiceImpl {
    pub fn new(repository: Arc<Mutex<Option<WorkerRepositoryImpl>>>) -> Self {
        WorkerServiceImpl { repository }
    }

    pub fn get_instance() -> Arc<Mutex<Option<WorkerServiceImpl>>> {
        INIT.call_once(|| {
            let repository = WorkerRepositoryImpl::get_instance();
            let instance = Arc::new(Mutex::new(Some(WorkerServiceImpl::new(repository.clone()))));
            *THREAD_SERVICE.lock().unwrap() = Some(WorkerServiceImpl::new(repository));
        });
        THREAD_SERVICE.clone()
    }
}

// Implement the ThreadServiceTrait for ThreadServiceImpl
impl WorkerServiceTrait for WorkerServiceImpl {
    fn create_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>) {
        // Lock the repository to get access to the inner value
        let repository_guard = self.repository.lock().unwrap();

        // Access the inner value and call save_thread
        if let Some(repository) = repository_guard.as_ref() {
            repository.save_thread(name, custom_function);
        }
    }

    fn get_thread(&self, name: &str) -> Option<Worker> {
        let repository_guard = self.repository.lock().unwrap();

        // Similar changes here
        repository_guard.as_ref().and_then(|repository| repository.get_thread(name))
    }

    fn start_worker(&self, name: &str) {
        let repository = self.repository.lock().unwrap();
        repository.as_ref().unwrap().start_worker_thread(name);
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;
    use crate::thread_manage_legacy::service::worker_service::WorkerServiceTrait;

    #[test]
    fn test_singleton_behavior() {
        // Initialize WorkerServiceImpl twice
        let instance1 = WorkerServiceImpl::get_instance();
        let instance2 = WorkerServiceImpl::get_instance();

        // Ensure that both instances point to the same underlying service
        assert!(Arc::ptr_eq(&instance1, &instance2));

        // Access the service through one instance and create a thread
        instance1.lock().unwrap().as_ref().unwrap().create_thread("Alice", None);

        // Access the service through the other instance and get the created thread
        let retrieved_worker = instance2.lock().unwrap().as_ref().unwrap().get_thread("Alice");

        // Verify that the thread was created successfully
        assert_eq!(retrieved_worker.map(|w| w.name().to_owned()), Some("Alice".to_owned()));
    }

    #[test]
    fn test_create_and_get_thread() {
        // Initialize WorkerServiceImpl
        let instance = WorkerServiceImpl::get_instance();

        // Create a thread
        instance.lock().unwrap().as_ref().unwrap().create_thread("Bob", None);

        // Get the created thread
        let retrieved_worker = instance.lock().unwrap().as_ref().unwrap().get_thread("Bob");

        // Verify that the thread was created successfully
        assert_eq!(retrieved_worker.map(|w| w.name().to_owned()), Some("Bob".to_owned()));
    }

    #[test]
    fn test_get_nonexistent_thread() {
        // Initialize WorkerServiceImpl
        let instance = WorkerServiceImpl::get_instance();

        // Try to get a thread that doesn't exist
        let retrieved_worker = instance.lock().unwrap().as_ref().unwrap().get_thread("Nonexistent");

        // Verify that the result is None
        assert_eq!(retrieved_worker, None);
    }

    #[test]
    fn test_start_worker_thread_with_custom_function() {
        // Create WorkerServiceImpl
        let instance = WorkerServiceImpl::get_instance();

        // Create a worker with a custom function
        let custom_function_executed = Arc::new(Mutex::new(false));
        let custom_function_executed_clone = Arc::clone(&custom_function_executed);
        let custom_function = move || {
            println!("Custom function executed!");
            // Set the flag to true when the function is executed
            *custom_function_executed_clone.lock().unwrap() = true;
        };
        let worker_name = "Alice";
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .create_thread(worker_name, Some(Box::new(custom_function.clone())));

        // Start the worker thread using the service
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .start_worker(worker_name);

        // Allow some time for the thread to execute
        thread::sleep(std::time::Duration::from_secs(1));

        // Check if the custom function was executed
        let lock = custom_function_executed.lock().unwrap();
        assert_eq!(*lock, true);
    }
}