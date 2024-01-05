use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::spawn_local;
use crate::thread_control::repository::thread_worker_repository::{CloneFunction, ThreadWorkerRepositoryTrait};
use crate::thread_control::repository::thread_worker_repository_impl::ThreadWorkerRepositoryImpl;
use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;

pub struct ThreadWorkerServiceImpl {
    repository: Arc<Mutex<ThreadWorkerRepositoryImpl>>,
}

impl ThreadWorkerServiceImpl {
    pub fn new(repository: Arc<Mutex<ThreadWorkerRepositoryImpl>>) -> Self {
        ThreadWorkerServiceImpl { repository }
    }

    pub fn get_instance() -> Arc<Mutex<ThreadWorkerServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ThreadWorkerServiceImpl>> =
                Arc::new(Mutex::new(ThreadWorkerServiceImpl::new(ThreadWorkerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ThreadWorkerServiceTrait for ThreadWorkerServiceImpl {
    async fn save_async_thread_worker(&mut self, name: &str, will_be_execute_function: Arc<Mutex<dyn CloneFunction>>) {
        let mut repository_mutex = self.repository.lock().await;

        repository_mutex.save_thread_worker(name, will_be_execute_function);
    }

    // fn save_sync_thread_worker(&mut self, name: &str, will_be_execute_function: Arc<Mutex<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>) {
    //     let sync_function = move || -> Pin<Box<dyn Future<Output = ()>>> {
    //         let will_be_execute_function = Arc::clone(&will_be_execute_function);
    //         Box::pin(async move {
    //             (will_be_execute_function.lock().unwrap())().await
    //         })
    //     };
    //
    //     self.repository.lock().unwrap().save_thread_worker(name, Some(Box::new(sync_function)));
    // }

    async fn start_thread_worker(&self, name: &str) {
        let repository_clone = Arc::clone(&self.repository);

        // Spawn a tokio task
        tokio::task::spawn(async move {
            // Lock the mutex and execute the method
            let arc_repo_clone = Arc::new(repository_clone);
            let repo_clone = arc_repo_clone.lock().await;
            repo_clone.start_thread_worker(name).await;
            // The lock is automatically released when the task completes
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[allow(dead_code)]
    fn my_sync_function() {
        println!("Synchronous function is executed!");
    }

    #[allow(dead_code)]
    async fn my_async_function() {
        println!("Asynchronous function is executed!");
    }

    #[test]
    async fn test_save_async_thread_worker() {
        let thread_worker_repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(thread_worker_repository);

        let async_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom async function executed!");
            })
        };

        service.save_async_thread_worker("AsyncTestWorker", Arc::new(Mutex::new(async_function)));

        if let Some(worker) = service.repository.lock().unwrap().find_by_name("AsyncTestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());
            let guard = function_arc.lock().await;
            let function = &*guard;

            let future = function();
            future.await;

            assert_eq!(worker.name(), "AsyncTestWorker");
        } else {
            panic!("Thread worker not found: AsyncTestWorker");
        };
    }

    #[test]
    async fn test_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(repository);

        let sync_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom sync function executed!");
            })
        };

        service.save_sync_thread_worker("SyncTestWorker", Arc::new(Mutex::new(sync_function)));

        if let Some(worker) = service.repository.lock().unwrap().find_by_name("SyncTestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());
            let guard = function_arc.lock().await;
            let function = &*guard;

            let future = function();
            future.await;

            assert_eq!(worker.name(), "SyncTestWorker");
        } else {
            panic!("Thread worker not found: SyncTestWorker");
        };
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_async_thread_and_start() {
        let thread_worker_repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(thread_worker_repository);

        let async_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom async function executed!");
            })
        };

        service.save_async_thread_worker("AsyncTestWorker", Arc::new(Mutex::new(async_function)));
        service.start_thread_worker("AsyncTestWorker").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_and_start() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(repository);

        let sync_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom sync function executed!");
            })
        };

        service.save_sync_thread_worker("SyncTestWorker", Arc::new(Mutex::new(sync_function)));
        service.start_thread_worker("SyncTestWorker").await;
    }

    #[test]
    async fn test_singleton() {
        let instance1 = ThreadWorkerServiceImpl::get_instance();
        let instance2 = ThreadWorkerServiceImpl::get_instance();

        assert_eq!(Arc::ptr_eq(&instance1, &instance2), true);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_and_start_with_singleton() {
        let service_instance = ThreadWorkerServiceImpl::get_instance();
        let mut service = service_instance.lock().unwrap();

        let sync_custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom sync function executed!");
            })
        };

        service.save_sync_thread_worker("SyncTestWorker", Arc::new(Mutex::new(sync_custom_function)));
        service.start_thread_worker("SyncTestWorker").await;
    }
}