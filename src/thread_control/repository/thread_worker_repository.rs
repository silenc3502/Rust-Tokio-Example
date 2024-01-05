use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::thread_control::entity::thread_worker::ThreadWorker;

#[async_trait]
pub trait CloneFunction: Send + Sync {
    async fn call(&self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

#[async_trait]
pub trait ThreadWorkerRepositoryTrait: Send + Sync {
    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Arc<Mutex<dyn CloneFunction>>,
    );
    fn find_by_name(&self, name: &str) -> Option<ThreadWorker>;
    async fn start_thread_worker(&'static self, name: &str);
}