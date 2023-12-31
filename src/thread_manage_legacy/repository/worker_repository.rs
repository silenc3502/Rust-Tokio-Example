use std::thread::ThreadId;
use crate::thread_manage_legacy::entity::worker::Worker;

// pub trait WorkerRepositoryTrait: Sync {
//     fn save_thread(&self, thread_id: ThreadId, name: &str, custom_function: Option<Box<dyn Fn()>>);
//     fn generate_thread_id(&self) -> ThreadId;
//     fn get_thread(&self, thread_id: ThreadId) -> Option<Worker>;
// }

// pub trait WorkerRepositoryTrait: Sync {
//     /// Saves a thread with the given parameters.
//     fn save_thread<F>(&self, thread_id: ThreadId, name: &str, custom_function: Option<F>)
//         where
//             F: Fn() + Send + 'static;
//
//     /// Generates a unique thread ID.
//     fn generate_thread_id(&self) -> ThreadId;
//
//     /// Retrieves information about a thread with the specified ID.
//     fn get_thread(&self, thread_id: ThreadId) -> Option<Worker>;
// }

pub trait WorkerRepositoryTrait: Sync {
    /// Saves a thread with the given parameters.
    fn save_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>);

    /// Retrieves information about a thread with the specified ID.
    fn get_thread(&self, name: &str) -> Option<Worker>;
    fn start_worker_thread(&self, name: &str);
}