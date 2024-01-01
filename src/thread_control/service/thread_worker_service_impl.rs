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
            // let instance = Arc::new(Mutex::new(Some(ThreadWorkerServiceImpl::new(repository.clone()))));
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

    fn start_worker(&self, name: &str) {
        let repository_guard = self.repository.lock().unwrap();

        if let Some(repository) = repository_guard.as_ref() {
            repository.start_thread_worker(name);
        } else {
            eprintln!("Repository is not initialized.");
        }
    }
}

pub fn get_thread_worker_service() -> &'static Mutex<Option<ThreadWorkerServiceImpl>> {
    INIT.call_once(|| {
        // ThreadWorkerRepositoryImpl을 생성합니다.
        let repository = ThreadWorkerRepositoryImpl::new();

        // ThreadWorkerServiceImpl을 생성하고 repository를 주입합니다.
        let service = ThreadWorkerServiceImpl::new(Arc::new(Mutex::new(Some(repository))));

        // 생성한 ThreadWorkerServiceImpl을 lazy_static!에서 관리하는 THREAD_SERVICE에 저장합니다.
        *THREAD_SERVICE.lock().unwrap() = Some(service);
    });
    &THREAD_SERVICE
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
    use crate::client_socket_accept::controller::client_socket_accept_controller_impl::get_client_socket_accept_controller;
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

    #[tokio::test]
    async fn test_start_worker_thread_with_custom_function() {
        let instance = ThreadWorkerServiceImpl::get_instance();

        let custom_function_executed = Arc::new(Mutex::new(false));
        let custom_function_executed_clone = Arc::clone(&custom_function_executed);
        let custom_function = move || {
            println!("Custom function executed!");
            *custom_function_executed_clone.lock().unwrap() = true;
        };
        let worker_name = "Alice";

        // Create thread with custom function
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .create_thread(worker_name, Some(Box::new(custom_function.clone())));

        // Start the worker thread
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

    #[tokio::test]
    async fn test_start_worker_thread_without_custom_function() {
        let instance = ThreadWorkerServiceImpl::get_instance();

        let custom_function_executed = Arc::new(Mutex::new(false));
        let custom_function_executed_clone = Arc::clone(&custom_function_executed);
        let worker_name = "Bob";

        // Create thread without custom function
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .create_thread(worker_name, None);

        // Start the worker thread
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .start_worker(worker_name);

        // Allow some time for the thread to execute
        thread::sleep(std::time::Duration::from_secs(1));

        // Check if the custom function was not executed
        let lock = custom_function_executed.lock().unwrap();
        assert_eq!(*lock, false);
    }

    #[tokio::test]
    async fn test_start_worker_thread_with_specific_object() {
        #[derive(Clone)]
        struct HelloObject;

        impl HelloObject {
            fn get_message(&self) -> &'static str {
                "Hello, World!"
            }
        }

        // Initialize the thread worker service
        let instance = get_thread_worker_service();

        // HelloObject instance
        let hello_object = HelloObject;

        // Custom function using HelloObject
        let custom_function = move || {
            println!("Custom function executed! Message: {}", hello_object.get_message());
        };
        let worker_name = "Alice";

        // Create thread with custom function
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .create_thread(worker_name, Some(Box::new(custom_function.clone())));

        // Start the worker thread
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .start_worker(worker_name);

        // Allow some time for the thread to execute
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    #[tokio::test]
    async fn test_start_worker_thread_with_locked_object() {
        lazy_static! {
            static ref HELLO_OBJECT_INSTANCE: Arc<Mutex<Option<HelloObject>>> = Arc::new(Mutex::new(None));
            static ref INIT: Once = Once::new();
        }

        pub struct HelloObject;

        impl HelloObject {
            pub fn new() -> Self {
                HelloObject
            }

            pub fn get_message(&self) -> &'static str {
                "Hello, World!"
            }
        }

        pub fn get_hello_object_instance() -> Arc<Mutex<Option<HelloObject>>> {
            INIT.call_once(|| {
                *HELLO_OBJECT_INSTANCE.lock().unwrap() = Some(HelloObject::new());
            });
            HELLO_OBJECT_INSTANCE.clone()
        }

        // Initialize the thread worker service
        let instance = get_thread_worker_service();

        let hello_object_instance = get_hello_object_instance();

        // Clone the HelloObject instance inside the worker function
        let custom_function = move || {
            let binding = hello_object_instance.lock().unwrap();
            let hello_object = binding.as_ref().unwrap().clone();
            println!("Custom function executed! Message: {}", hello_object.get_message());
        };
        let worker_name = "Alice";

        // Create thread with custom function
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .create_thread(worker_name, Some(Box::new(custom_function.clone())));

        // Start the worker thread
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .start_worker(worker_name);

        // Allow some time for the thread to execute
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    #[tokio::test]
    async fn test_start_worker_thread_with_dependency_call() {
        pub trait HelloObjectService: Send {
            fn get_message(&self) -> &'static str;
        }

        // HelloObjectServiceImpl as an implementation of HelloObjectService
        pub struct HelloObjectServiceImpl;

        impl HelloObjectService for HelloObjectServiceImpl {
            fn get_message(&self) -> &'static str {
                println!("Service Call");
                "Hello, World!"
            }
        }

        impl Clone for HelloObjectServiceImpl {
            fn clone(&self) -> Self {
                HelloObjectServiceImpl
            }
        }

        // HelloObject with a dependency on HelloObjectService
        #[derive(Clone)]
        pub struct HelloObject {
            // The dependency on HelloObjectService
            service: Arc<Mutex<Option<Box<dyn HelloObjectService>>>>,
        }

        impl HelloObject {
            pub fn new() -> Self {
                HelloObject {
                    service: Arc::new(Mutex::new(None)),
                }
            }

            // Set the HelloObjectService dependency
            pub fn set_service(&self, service: Box<dyn HelloObjectService>) {
                *self.service.lock().unwrap() = Some(service);
            }

            // Call a function from the dependency
            pub fn dependency_call(&self) -> &'static str {
                println!("Controller Call");
                // Access the dependency and call its function
                let service = self.service.lock().unwrap();
                service.as_ref().map_or("Dependency not set", |s| s.get_message())
            }

            pub fn clone(&self) -> Self {
                HelloObject {
                    service: Arc::clone(&self.service),
                }
            }
        }

        // Lazy initialization for HelloObjectService
        lazy_static! {
            static ref HELLO_OBJECT_SERVICE_INSTANCE: Arc<Mutex<Option<HelloObjectServiceImpl>>> =
                Arc::new(Mutex::new(None));
            static ref INIT: Once = Once::new();
        }

        // Initialize the thread worker service
        let instance = get_thread_worker_service();

        INIT.call_once(|| {
            *HELLO_OBJECT_SERVICE_INSTANCE.lock().unwrap() = Some(HelloObjectServiceImpl);
        });

        let hello_object_instance = HELLO_OBJECT_SERVICE_INSTANCE.clone();

        let hello_object = HelloObject::new();
        hello_object.set_service(Box::new(hello_object_instance.lock().unwrap().as_ref().unwrap().clone()));

        let custom_function = move || {
            println!("Custom function executed! Message: {}", hello_object.dependency_call());
        };

        let worker_name = "Alice";

        // Create thread with custom function
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .create_thread(worker_name, Some(Box::new(custom_function.clone())));

        // Start the worker thread
        instance
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .start_worker(worker_name);

        // Allow some time for the thread to execute
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}