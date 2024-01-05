use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::thread_control::repository::thread_worker_repository::CloneFunction;

#[async_trait]
pub trait ThreadWorkerServiceTrait {
    async fn save_async_thread_worker(&mut self, name: &str, will_be_execute_function: Arc<Mutex<dyn CloneFunction>>);
    // fn save_sync_thread_worker(&mut self, name: &str, will_be_execute_function: Arc<Mutex<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>);
    async fn start_thread_worker(&self, name: &str);
}

