// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::sync::Once;
// use std::thread;
// use lazy_static::lazy_static;
// use crate::thread_manage_legacy::entity::worker::Worker;
// use crate::thread_manage_legacy::repository::worker_repository::WorkerRepositoryTrait;
//
// #[derive(Debug, Clone)]
// pub struct WorkerRepositoryImpl {
//     workers: Arc<Mutex<HashMap<String, Worker>>>,
// }
//
// lazy_static! {
//     static ref WORKER_REPOSITORY: Arc<Mutex<Option<WorkerRepositoryImpl>>> = Arc::new(Mutex::new(None));
//     static ref INIT: Once = Once::new();
// }
//
// fn initialize_repository() -> WorkerRepositoryImpl {
//     WorkerRepositoryImpl {
//         workers: Arc::new(Mutex::new(HashMap::new())),
//     }
// }
//
// impl WorkerRepositoryImpl {
//     pub fn new() -> Self {
//         initialize_repository()
//     }
//
//     pub fn get_instance() -> Arc<Mutex<Option<WorkerRepositoryImpl>>> {
//         INIT.call_once(|| {
//             *WORKER_REPOSITORY.lock().unwrap() = Some(initialize_repository());
//         });
//         WORKER_REPOSITORY.clone()
//     }
// }
//
// impl WorkerRepositoryTrait for WorkerRepositoryImpl {
//     fn save_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>) {
//         let mut workers = self.workers.lock().unwrap();
//         let worker = Worker::new(name, custom_function);
//         workers.insert(name.to_string(), worker);
//     }
//
//     fn get_thread(&self, name: &str) -> Option<Worker> {
//         let workers = self.workers.lock().unwrap();
//         workers.get(name).cloned()
//     }
//
//     fn start_worker_thread(&self, name: &str) {
//         if let Some(worker) = self.get_thread(name) {
//             if let Some(custom_function) = worker.cloned_custom_function() {
//                 let name_owned = name.to_string(); // 소유권 이전
//                 thread::spawn(move || {
//                     println!("Starting worker thread for: {}", name_owned);
//                     custom_function();
//                     println!("Worker thread finished for: {}", name_owned);
//                 });
//             } else {
//                 println!("No custom function available for worker: {}", name);
//             }
//         } else {
//             println!("Worker not found: {}", name);
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use std::sync::atomic::{AtomicBool, Ordering};
//     use std::sync::Weak;
//     use super::*;
//
//     #[test]
//     fn test_singleton_behavior() {
//         let repository1 = WorkerRepositoryImpl::new();
//         let repository2 = repository1.clone();
//
//         assert!(Arc::ptr_eq(&repository1.workers, &repository2.workers));
//     }
//
//     #[test]
//     fn test_save_thread() {
//         let repository = WorkerRepositoryImpl::new();
//         repository.save_thread("John Doe", None);
//
//         let worker = repository.get_thread("John Doe").expect("Worker not found.");
//         assert_eq!(worker.name(), "John Doe");
//         assert!(worker.cloned_custom_function().is_none()); // 수정된 부분
//     }
//
//     #[test]
//     fn test_save_thread_with_custom_function() {
//         let repository = WorkerRepositoryImpl::new();
//         let custom_function = || {
//             println!("Custom function executed!");
//         };
//
//         repository.save_thread("John Doe", Some(Box::new(custom_function)));
//
//         let worker = repository.get_thread("John Doe").expect("Worker not found.");
//         assert_eq!(worker.name(), "John Doe");
//         assert!(worker.cloned_custom_function().is_some());
//     }
//
//     #[test]
//     fn test_get_thread_nonexistent() {
//         let repository = WorkerRepositoryImpl::new();
//
//         let worker = repository.get_thread("Nonexistent");
//         assert!(worker.is_none());
//     }
//
//     #[test]
//     fn test_get_thread_existing() {
//         let repository = WorkerRepositoryImpl::new();
//         repository.save_thread("Jane Doe", None);
//
//         let worker = repository.get_thread("Jane Doe").expect("Worker not found.");
//         assert_eq!(worker.name(), "Jane Doe");
//     }
//
//     #[test]
//     fn test_start_worker_thread_with_custom_function() {
//         // Create a worker with a custom function
//         let custom_function_executed = Arc::new(Mutex::new(false));
//         let custom_function_executed_clone = Arc::clone(&custom_function_executed);
//         let custom_function = move || {
//             println!("Custom function executed!");
//             // Set the flag to true when the function is executed
//             let mut lock = custom_function_executed_clone.lock().unwrap();
//             *lock = true;
//         };
//         let worker = Worker::new("Alice", Some(Box::new(custom_function.clone())));
//
//         // Create a repository with the worker
//         let repository = WorkerRepositoryImpl::new();
//         repository.save_thread("Alice", Some(Box::new(custom_function)));
//
//         // Start the worker thread using the repository
//         repository.start_worker_thread("Alice");
//
//         // Allow some time for the thread to execute
//         thread::sleep(std::time::Duration::from_secs(1));
//
//         // Check if the custom function was executed
//         let lock = custom_function_executed.lock().unwrap();
//         assert_eq!(*lock, true);
//     }
//
//     #[test]
//     fn test_start_worker_thread_without_custom_function() {
//         // Create a worker without a custom function
//         let worker = Worker::new("Bob", None);
//
//         // Create a repository with the worker
//         let repository = WorkerRepositoryImpl::new();
//         repository.save_thread("Bob", None);
//
//         // Start the worker thread using the repository
//         repository.start_worker_thread("Bob");
//
//         // No custom function, so the thread won't do anything
//     }
// }
//
//
