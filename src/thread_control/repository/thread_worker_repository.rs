use crate::thread_control::entity::thread_worker::ThreadWorker;

pub trait ThreadWorkerRepositoryTrait: Sync {
    fn save_thread_worker(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>);
    fn find_by_name(&self, name: &str) -> Option<ThreadWorker>;
    // fn start_worker_thread(&self, name: &str);
}