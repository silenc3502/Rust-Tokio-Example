use crate::thread_control::entity::thread_worker::ThreadWorker;

pub trait ThreadWorkerServiceTrait {
    fn create_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>);
    fn get_thread(&self, name: &str) -> Option<ThreadWorker>;
    fn start_worker(&self, name: &str);
}