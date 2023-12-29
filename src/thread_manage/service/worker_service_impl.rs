// use std::sync::{Arc, Mutex};
// use std::thread::ThreadId;
// use lazy_static::lazy_static;
// use crate::thread_manage::entity::worker::Worker;
// use crate::thread_manage::service::worker_service::WorkerServiceTrait;
// use crate::thread_manage::repository::worker_repository::WorkerRepositoryTrait;
// use crate::thread_manage::repository::worker_repository_impl::WorkerRepositoryImpl;
//
// lazy_static! {
//     static ref WORKER_REPOSITORY: WorkerRepositoryImpl = WorkerRepositoryImpl::new();
// }
//
// pub struct WorkerServiceImpl {
//     worker_repository: &'static WorkerRepositoryImpl,
// }
//
// impl WorkerServiceImpl {
//     pub fn new(worker_repository: &'static WorkerRepositoryImpl) -> Self {
//         WorkerServiceImpl { worker_repository }
//     }
// }
//
// impl WorkerServiceTrait for WorkerServiceImpl {
//     fn create_thread(&self, name: &str) -> ThreadId {
//         let thread_id = self.worker_repository.generate_thread_id();
//         self.worker_repository.save_thread(thread_id, name);
//         thread_id
//     }
//
//     fn get_thread(&self, thread_id: ThreadId) -> Option<Worker> {
//         self.worker_repository.get_thread(thread_id)
//     }
//
// }
//
// lazy_static! {
//     static ref WORKER_SERVICE: Arc<Mutex<WorkerServiceImpl>> =
//         Arc::new(Mutex::new(WorkerServiceImpl::new(&*WORKER_REPOSITORY)));
// }
//
// pub fn get_worker_service() -> Arc<Mutex<WorkerServiceImpl>> {
//     Arc::clone(&WORKER_SERVICE)
// }
//
// #[cfg(test)]
// mod singleton_behavior_tests {
//     use super::*;
//     use std::sync::{Arc, Mutex};
//
//     fn setup_create_thread_test() -> (WorkerServiceImpl, &'static str) {
//         let worker_service = WorkerServiceImpl::new(&*WORKER_REPOSITORY);
//         let name = "TestThread";
//         (worker_service, name)
//     }
//
//     #[test]
//     fn test_singleton_behavior() {
//         let service1 = get_worker_service();
//         let service2 = get_worker_service();
//
//         assert!(Arc::ptr_eq(&service1, &service2));
//     }
//
//     #[test]
//     fn test_inner_singleton_behavior() {
//         let service1 = get_worker_service();
//         let inner_service1 = service1.lock().unwrap();
//
//         let repository1 = inner_service1.worker_repository.clone();
//
//         drop(inner_service1); // unlock
//
//         let service2 = get_worker_service();
//         let inner_service2 = service2.lock().unwrap();
//
//         assert!(Arc::ptr_eq(&repository1.threads, &inner_service2.worker_repository.threads));
//     }
//
//     #[test]
//     fn test_create_and_get_thread() {
//         let (worker_service, name) = setup_create_thread_test();
//         let thread_id = worker_service.create_thread(name);
//         let retrieved_thread = worker_service.get_thread(thread_id);
//
//         assert!(retrieved_thread.is_some());
//         assert_eq!(retrieved_thread.unwrap().name(), name);
//     }
// }


use std::sync::{Arc, Mutex, Once};
use lazy_static::lazy_static;
use crate::thread_manage::entity::worker::Worker;
use crate::thread_manage::repository::worker_repository::WorkerRepositoryTrait;
use crate::thread_manage::repository::worker_repository_impl::WorkerRepositoryImpl;
use crate::thread_manage::service::worker_service::WorkerServiceTrait;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thread_manage::service::worker_service::WorkerServiceTrait;

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
}