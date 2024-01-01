// use crate::thread_manage_legacy::entity::worker::Worker;
//
// pub trait WorkerRepositoryTrait: Sync {
//     /// Saves a thread with the given parameters.
//     fn save_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>);
//
//     /// Retrieves information about a thread with the specified ID.
//     fn get_thread(&self, name: &str) -> Option<Worker>;
//     fn start_worker_thread(&self, name: &str);
// }