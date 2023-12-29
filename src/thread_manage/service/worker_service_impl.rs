use std::sync::{Arc, Mutex};
use std::thread::ThreadId;
use lazy_static::lazy_static;
use crate::thread_manage::entity::worker::Worker;
use crate::thread_manage::service::worker_service::WorkerServiceTrait;
use crate::thread_manage::repository::worker_repository::WorkerRepositoryTrait;
use crate::thread_manage::repository::worker_repository_impl::WorkerRepositoryImpl;

lazy_static! {
    static ref WORKER_REPOSITORY: WorkerRepositoryImpl = WorkerRepositoryImpl::new();
}

pub struct WorkerServiceImpl {
    worker_repository: &'static WorkerRepositoryImpl,
}

impl WorkerServiceImpl {
    pub fn new(worker_repository: &'static WorkerRepositoryImpl) -> Self {
        WorkerServiceImpl { worker_repository }
    }
}

impl WorkerServiceTrait for WorkerServiceImpl {
    fn create_thread(&self, name: &str) -> ThreadId {
        let thread_id = self.worker_repository.generate_thread_id();
        self.worker_repository.save_thread(thread_id, name);
        thread_id
    }

    fn get_thread(&self, thread_id: ThreadId) -> Option<Worker> {
        self.worker_repository.get_thread(thread_id)
    }

}

lazy_static! {
    static ref WORKER_SERVICE: Arc<Mutex<WorkerServiceImpl>> =
        Arc::new(Mutex::new(WorkerServiceImpl::new(&*WORKER_REPOSITORY)));
}

pub fn get_worker_service() -> Arc<Mutex<WorkerServiceImpl>> {
    Arc::clone(&WORKER_SERVICE)
}

#[cfg(test)]
mod singleton_behavior_tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn setup_create_thread_test() -> (WorkerServiceImpl, &'static str) {
        let worker_service = WorkerServiceImpl::new(&*WORKER_REPOSITORY);
        let name = "TestThread";
        (worker_service, name)
    }

    #[test]
    fn test_singleton_behavior() {
        let service1 = get_worker_service();
        let service2 = get_worker_service();

        assert!(Arc::ptr_eq(&service1, &service2));
    }

    #[test]
    fn test_inner_singleton_behavior() {
        let service1 = get_worker_service();
        let inner_service1 = service1.lock().unwrap();

        let repository1 = inner_service1.worker_repository.clone();

        drop(inner_service1); // unlock

        let service2 = get_worker_service();
        let inner_service2 = service2.lock().unwrap();

        assert!(Arc::ptr_eq(&repository1.threads, &inner_service2.worker_repository.threads));
    }

    #[test]
    fn test_create_and_get_thread() {
        let (worker_service, name) = setup_create_thread_test();
        let thread_id = worker_service.create_thread(name);
        let retrieved_thread = worker_service.get_thread(thread_id);

        assert!(retrieved_thread.is_some());
        assert_eq!(retrieved_thread.unwrap().name(), name);
    }
}
