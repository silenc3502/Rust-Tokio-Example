use crate::thread_manage::entity::worker::Worker;

pub trait WorkerServiceTrait {
    fn create_thread(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>);
    fn get_thread(&self, name: &str) -> Option<Worker>;
    fn start_worker(&self, name: &str);
}