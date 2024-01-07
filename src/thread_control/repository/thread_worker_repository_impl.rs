use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use tokio::{spawn, task};
use crate::thread_control::entity::closure::Closure;
use crate::thread_control::entity::thread_worker::ThreadWorker;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;

// trait CloneFunction: Send {
//     fn call(&self) -> Pin<Box<dyn Future<Output = ()>>>;
// }
//
// impl<T> CloneFunction for T
//     where
//         T: 'static + Fn() -> Pin<Box<dyn futures::Future<Output = ()>>> + Send,
// {
//     fn call(&self) -> Pin<Box<dyn futures::Future<Output = ()>>> {
//         (self)()
//     }
// }

pub struct ThreadWorkerRepositoryImpl {
    thread_worker_list: Arc<Mutex<HashMap<String, ThreadWorker>>>,
}

impl ThreadWorkerRepositoryImpl {
    pub fn new() -> Self {
        ThreadWorkerRepositoryImpl {
            thread_worker_list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_instance() -> Arc<Mutex<ThreadWorkerRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ThreadWorkerRepositoryImpl>> = {
                // 클로저에서 사용하는 데이터가 Send를 만족하도록 보장
                let shared_data = Arc::new(Mutex::new(ThreadWorkerRepositoryImpl::new()));

                // 클로저 내에서 Arc<Mutex<...>> 를 복제하여 반환
                Arc::clone(&shared_data)
            };
        }
        INSTANCE.clone()
    }


    // pub fn get_thread_worker_list(&self) -> &HashMap<String, ThreadWorker> {
    //     &self.thread_worker_list.lock().unwrap()
    // }
}

#[async_trait]
impl ThreadWorkerRepositoryTrait for ThreadWorkerRepositoryImpl {

    // fn save_thread_worker(
    //     &mut self,
    //     name: &str,
    //     will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
    // ) {
    //     let thread_worker = ThreadWorker::new(name, will_be_execute_function);
    //     self.thread_worker_list.insert(name.to_string(), thread_worker);
    // }

    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Option<Closure>,
    ) {
        let mut thread_worker_list = self.thread_worker_list.lock().unwrap();

        let mut thread_worker = ThreadWorker::new(name);
        thread_worker.save_function(will_be_execute_function.unwrap());

        thread_worker_list.insert(name.to_string(), thread_worker);
    }

    // fn find_by_name(&self, name: &str) -> Option<ThreadWorker> {
    //     let thread_worker_list = self.thread_worker_list.lock().expect("Mutex lock failed");
    //
    //     if let Some(worker) = thread_worker_list.get(name) {
    //         Some(worker.clone())
    //     } else {
    //         None
    //     }
    // }

    // async fn start_thread_worker(&self, name: &str) {
    //     let thread_worker_list = self.get_thread_worker_list();
    //
    //     if let Some(worker) = thread_worker_list.get(name) {
    //         if let Some(function_arc_ref) = worker.get_will_be_execute_function_ref() {
    //             // function_arc_ref의 타입: &Arc<Mutex<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>>
    //             let guard = function_arc_ref.lock().await;
    //
    //             let guard_deref = &*guard;
    //             let real_function = &**guard_deref;
    //
    //             // 1.
    //             let future = real_function();
    //
    //             task::block_in_place(move || {
    //                 Handle::current().block_on(async move {
    //                     future.await
    //                 });
    //             });
    //
    //             // 2.
    //             // let function = function_arc_ref.lock().await.clone();
    //             //
    //             // task::spawn(async move {
    //             //     function().await;
    //             // }).await.unwrap();
    //
    //             // 3.
    //             // let function = CloneFunction::clone_function(guard.as_ref());
    //             //
    //             // tokio::spawn(async move {
    //             //     let function = CloneFunction::clone_function(guard.as_ref());
    //             //     function().await;
    //             // }).await.unwrap();
    //
    //             assert_eq!(worker.name(), name);
    //         } else {
    //             panic!("Thread worker function not found: {}", name);
    //         }
    //     } else {
    //         panic!("Thread worker not found: {}", name);
    //     }
    // }
    async fn start_thread_worker(&self, name: &str) {
        // let thread_worker_list = self.get_thread_worker_list();

        if let Some(worker) = self.thread_worker_list.lock().unwrap().get_mut(name) {
            println!("check worker name: {}", worker.name());
            if let Some(closure) = worker.get_will_be_execute_function() {
                println!("found closure!");
                spawn(closure.0);
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

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        let custom_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        repository.save_thread_worker("TestWorker", Some(Closure::Async(Box::new(custom_function.clone()))));
        repository.start_thread_worker("TestWorker").await;

        // // Retrieve the saved worker and execute its function
        // if let Some(mut worker) = repository.thread_worker_list.lock().unwrap().get("TestWorker") {
        //     // Ensure the closure is Async before executing
        //     if let Some(Closure::Async(async_function)) = worker.get_will_be_execute_function() {
        //         // Execute the closure asynchronously
        //         async_function().await;
        //     } else {
        //         // Handle other closure types or do nothing for Sync closures
        //         // You might want to log a warning or handle differently based on your needs
        //         // For now, it doesn't panic when Sync closures are encountered.
        //     }
        //
        //     // Add an assertion to check if the worker name matches
        //     assert_eq!(worker.name(), "TestWorker");
        // } else {
        //     panic!("Thread worker not found!");
        // }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        // Synchronous function
        let sync_custom_function = || {
            my_sync_function();
        };

        // repository.save_thread_worker("TestWorker", Some(Closure::Async(Box::new(custom_function.clone()))));
        // repository.start_thread_worker("TestWorker").await;

        // Save a thread worker with a synchronous function
        repository.save_thread_worker("SyncTestWorker", Some(Closure::Sync(Box::new(sync_custom_function))));
        repository.start_thread_worker("SyncTestWorker").await;

        // // Retrieve and execute the saved worker's function
        // if let Some(worker) = repository.thread_worker_list.lock().unwrap().get("SyncTestWorker") {
        //     let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());
        //
        //     // Lock the Mutex to get the guard
        //     let guard = function_arc.lock().await;
        //
        //     // Extract the closure from the Box inside the Mutex guard
        //     let function = &*guard;
        //
        //     // Call the closure and execute the future
        //     let future = function();
        //     future.await;
        //
        //     // Add an assertion to check if the worker name matches
        //     assert_eq!(worker.name(), "SyncTestWorker");
        // } else {
        //     panic!("Thread worker not found: SyncTestWorker");
        // }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_async_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        // Asynchronous function
        let async_custom_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                my_async_function().await;
            })
        };

        repository.save_thread_worker("AsyncTestWorker", Some(Closure::Async(Box::new(async_custom_function))));
        repository.start_thread_worker("AsyncTestWorker").await;
    }
}
