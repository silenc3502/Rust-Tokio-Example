use std::thread::ThreadId;
use crate::thread_manage::entity::worker::Worker;

pub trait WorkerRepositoryTrait: Sync {
    fn save_thread(&self, thread_id: ThreadId, name: &str);
    fn generate_thread_id(&self) -> ThreadId;
    fn get_thread(&self, thread_id: ThreadId) -> Option<Worker>;
}