use std::thread::ThreadId;
use crate::thread_manage::entity::worker::Worker;

pub trait WorkerServiceTrait {
    fn create_thread(&self, name: &str) -> ThreadId;
    fn get_thread(&self, thread_id: ThreadId) -> Option<Worker>;
}