use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::task;
use crate::thread_control::entity::thread_worker::ThreadWorker;
use crate::thread_control::repository::thread_worker_repository::{CloneFunction, ThreadWorkerRepositoryTrait};

impl dyn CloneFunction {
    fn boxed_clone_function<F, Fut>(func: F) -> Box<dyn CloneFunction>
        where
            F: 'static + Fn() -> Fut + Send,
            Fut: Future<Output = ()> + Send + 'static,
    {
        Box::new(Arc::new(Mutex::new(func))) as Box<dyn CloneFunction>
    }
}

#[async_trait]
impl<F, Fut> CloneFunction for Arc<Mutex<F>>
    where
        F: 'static + Fn() -> Fut + Send,
        Fut: Future<Output = ()> + Send + 'static,
{
    async fn call(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let guard = self.lock().await;
        let future = (guard)();
        Box::pin(future)
    }
}

pub struct ThreadWorkerRepositoryImpl {
    thread_worker_list: HashMap<String, ThreadWorker>,
}

impl ThreadWorkerRepositoryImpl {
    pub fn new() -> Self {
        ThreadWorkerRepositoryImpl {
            thread_worker_list: HashMap::new(),
        }
    }

    pub fn get_instance() -> Arc<Mutex<ThreadWorkerRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ThreadWorkerRepositoryImpl>> =
                Arc::new(Mutex::new(ThreadWorkerRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    pub fn get_thread_worker_list(&self) -> &HashMap<String, ThreadWorker> {
        &self.thread_worker_list
    }
}

#[async_trait]
impl ThreadWorkerRepositoryTrait for ThreadWorkerRepositoryImpl {
    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Arc<Mutex<dyn CloneFunction>>,
    ) {
        let thread_worker = ThreadWorker::new(name, will_be_execute_function);
        self.thread_worker_list.insert(name.to_string(), thread_worker);
    }

    fn find_by_name(&self, name: &str) -> Option<ThreadWorker> {
        self.thread_worker_list.get(name).cloned()
    }

    async fn start_thread_worker(&'static self, name: &str) {
        let thread_worker_list = self.get_thread_worker_list();

        if let Some(worker) = thread_worker_list.get(name) {
            if let Some(function_arc_ref) = worker.get_will_be_execute_function_ref() {
                // function_arc_ref의 타입: &Arc<Mutex<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>>
                let guard = function_arc_ref.lock().await;

                // let guard_deref = &*guard;
                // let real_function = &**guard_deref;
                //
                // // 1.
                // let future = real_function();
                //
                // task::block_in_place(move || {
                //     Handle::current().block_on(async move {
                //         future.await
                //     });
                // });

                // 2.
                // let function = function_arc_ref.lock().await.clone();
                //
                // task::spawn(async move {
                //     function().await;
                // }).await.unwrap();

                // 3.
                // let function = CloneFunction::clone_function(guard.as_ref());
                //
                // tokio::spawn(async move {
                //     let function = CloneFunction::clone_function(guard.as_ref());
                //     function().await;
                // }).await.unwrap();

                // 4.

                let cloned_name = name.to_string(); // Clone the name

                task::spawn(async move {
                    let guard = function_arc_ref.lock().await;
                    let future = guard.call();

                    future.await;

                    assert_eq!(worker.name(), cloned_name); // Use cloned_name here
                })
                    .await
                    .expect("TODO: panic message");

                assert_eq!(worker.name(), name);
            } else {
                panic!("Thread worker function not found: {}", name);
            }
        } else {
            panic!("Thread worker not found: {}", name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    fn my_sync_function() {
        println!("Synchronous function is executed!");
    }

    async fn my_async_function() {
        println!("Asynchronous function is executed!");
    }

    #[test]
    async fn test_singleton() {
        let instance1 = ThreadWorkerRepositoryImpl::get_instance();
        let instance2 = ThreadWorkerRepositoryImpl::get_instance();

        // Ensure that both instances are the same
        assert_eq!(Arc::ptr_eq(&instance1, &instance2), true);
    }

    #[tokio::test]
    async fn test_save_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        // Save a thread worker
        repository.save_thread_worker("TestWorker", Some(Box::new(custom_function)));

        // Retrieve the saved worker and execute its function
        if let Some(worker) = repository.thread_worker_list.get("TestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());

            // Lock the Mutex to get the guard
            let guard = function_arc.lock().await;

            // Extract the closure from the Box inside the Mutex guard
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), "TestWorker");
        } else {
            panic!("Thread worker not found!");
        }
    }

    #[tokio::test]
    async fn test_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        // Synchronous function
        let sync_custom_function = || {
            Box::pin(async {
                my_sync_function();
            }) as Pin<Box<dyn Future<Output = ()>>>
        };

        // Save a thread worker with a synchronous function
        repository.save_thread_worker("SyncTestWorker", Some(Box::new(sync_custom_function)));

        // Retrieve and execute the saved worker's function
        if let Some(worker) = repository.thread_worker_list.get("SyncTestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());

            // Lock the Mutex to get the guard
            let guard = function_arc.lock().await;

            // Extract the closure from the Box inside the Mutex guard
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), "SyncTestWorker");
        } else {
            panic!("Thread worker not found: SyncTestWorker");
        }
    }

    #[tokio::test]
    async fn test_save_async_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        // Asynchronous function
        let async_custom_function = || {
            Box::pin(async {
                my_async_function().await;
            }) as Pin<Box<dyn Future<Output = ()>>>
        };

        // Save a thread worker with an asynchronous function
        repository.save_thread_worker("AsyncTestWorker", Some(Box::new(async_custom_function)));

        // Retrieve and execute the saved worker's function
        if let Some(worker) = repository.thread_worker_list.get("AsyncTestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());

            // Lock the Mutex to get the guard
            let guard = function_arc.lock().await;

            // Extract the closure from the Box inside the Mutex guard
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), "AsyncTestWorker");
        } else {
            panic!("Thread worker not found: AsyncTestWorker");
        }
    }

    #[tokio::test]
    async fn test_shared_thread_worker_list() {
        let instance1 = ThreadWorkerRepositoryImpl::get_instance();
        let instance2 = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository through instance1
        let mut repository1 = instance1.lock().unwrap();

        // Save a thread worker through instance1
        repository1.save_thread_worker("SharedTestWorker", None);

        // Drop the lock to allow other instances to access the repository
        drop(repository1);

        // Lock the mutex to access the repository through instance2
        let repository2 = instance2.lock().unwrap();

        // Check if the saved worker is visible through instance2
        assert!(repository2.thread_worker_list.contains_key("SharedTestWorker"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_new_save_thread_worker() {
        // Execute the async code within the tokio runtime
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        let custom_function = || -> Pin<Box<dyn Future<Output=()>>>   {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        // tokio::runtime::Builder::new_multi_thread()
        //     .worker_threads(1)
        //     .enable_all()
        //     .build()
        //     .unwrap()
        //     .block_on(async {
        //         custom_function
        //     });

        // Save a thread worker
        repository.save_thread_worker("TestWorker", Some(Box::new(custom_function)));

        repository.start_thread_worker("TestWorker").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_new_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        // Synchronous function
        let sync_custom_function = || {
            Box::pin(async {
                my_sync_function();
            }) as Pin<Box<dyn Future<Output = ()>>>
        };

        // Save a thread worker with a synchronous function
        repository.save_thread_worker("SyncTestWorker", Some(Box::new(sync_custom_function)));
        repository.start_thread_worker("SyncTestWorker").await;
    }
}
