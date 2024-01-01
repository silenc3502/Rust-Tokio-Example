use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use lazy_static::lazy_static;
use tokio::task;
use crate::thread_control::entity::thread_worker::ThreadWorker;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;

#[derive(Debug, Clone)]
pub struct ThreadWorkerRepositoryImpl {
    thread_worker_list: Arc<Mutex<HashMap<String, ThreadWorker>>>,
}

lazy_static! {
    static ref WORKER_REPOSITORY: Arc<Mutex<Option<ThreadWorkerRepositoryImpl>>> = Arc::new(Mutex::new(None));
    static ref INIT: Once = Once::new();
}

fn initialize_repository() -> ThreadWorkerRepositoryImpl {
    ThreadWorkerRepositoryImpl {
        thread_worker_list: Arc::new(Mutex::new(HashMap::new())),
    }
}

impl ThreadWorkerRepositoryImpl {
    pub fn new() -> Self {
        initialize_repository()
    }

    pub fn get_instance() -> Arc<Mutex<Option<ThreadWorkerRepositoryImpl>>> {
        INIT.call_once(|| {
            *WORKER_REPOSITORY.lock().unwrap() = Some(initialize_repository());
        });
        WORKER_REPOSITORY.clone()
    }
}

impl ThreadWorkerRepositoryTrait for ThreadWorkerRepositoryImpl {
    fn save_thread_worker(&self, name: &str, custom_function: Option<Box<dyn Fn() + Send + 'static>>) {
        let mut thread_worker_list = self.thread_worker_list.lock().unwrap();
        let thread_worker = ThreadWorker::new(name, custom_function);
        thread_worker_list.insert(name.to_string(), thread_worker);
    }

    fn find_by_name(&self, name: &str) -> Option<ThreadWorker> {
        let thread_worker_list = self.thread_worker_list.lock().unwrap();
        thread_worker_list.get(name).cloned()
    }

    fn start_thread_worker(&self, name: &str) {
        // 이름으로 Worker를 찾아옵니다.
        if let Some(worker) = self.find_by_name(name) {
            println!("Worker start!");
            // Worker의 클론된 함수를 가져와서 Tokio의 spawn_blocking을 이용해 동기적으로 실행합니다.
            if let Some(cloned_function) = worker.cloned_custom_function() {
                println!("cloned_function() start!");
                task::spawn_blocking(move || {
                    cloned_function();
                });
            } else {
                // 클론된 함수가 없다면 에러를 처리하거나 다른 방법으로 처리합니다.
                eprintln!("Worker {} does not have a custom function.", name);
            }
        } else {
            // Worker를 찾을 수 없다면 에러를 처리하거나 다른 방법으로 처리합니다.
            eprintln!("Worker {} not found.", name);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Weak;
    use std::thread;
    use std::time::Duration;
    use tokio::time::sleep;
    use super::*;

    #[test]
    fn test_singleton_behavior() {
        let repository1 = ThreadWorkerRepositoryImpl::new();
        let repository2 = repository1.clone();

        assert!(Arc::ptr_eq(&repository1.thread_worker_list, &repository2.thread_worker_list));
    }

    #[test]
    fn test_save_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::new();
        repository.save_thread_worker("John Doe", None);

        let thread_worker = repository.find_by_name("John Doe").expect("Worker not found.");
        assert_eq!(thread_worker.name(), "John Doe");
        assert!(thread_worker.cloned_custom_function().is_none()); // 수정된 부분
    }

    #[test]
    fn test_save_thread_with_custom_function() {
        let repository = ThreadWorkerRepositoryImpl::new();
        let custom_function = || {
            println!("Custom function executed!");
        };

        repository.save_thread_worker("John Doe", Some(Box::new(custom_function)));

        let thread_worker = repository.find_by_name("John Doe").expect("Worker not found.");
        assert_eq!(thread_worker.name(), "John Doe");
        assert!(thread_worker.cloned_custom_function().is_some());
    }

    #[test]
    fn test_get_thread_nonexistent() {
        let repository = ThreadWorkerRepositoryImpl::new();

        let thread_worker = repository.find_by_name("Nonexistent");
        assert!(thread_worker.is_none());
    }

    #[test]
    fn test_get_thread_existing() {
        let repository = ThreadWorkerRepositoryImpl::new();
        repository.save_thread_worker("Jane Doe", None);

        let thread_worker = repository.find_by_name("Jane Doe").expect("Worker not found.");
        assert_eq!(thread_worker.name(), "Jane Doe");
    }

    #[tokio::test]
    async fn test_start_thread_worker() {
        let custom_function_executed = Arc::new(Mutex::new(false));
        let custom_function_executed_clone = Arc::clone(&custom_function_executed);

        // 테스트를 위한 ThreadWorkerRepositoryImpl 인스턴스 생성
        let repository = ThreadWorkerRepositoryImpl::new();

        // 클로저를 따로 변수에 저장
        let custom_function = move || {
            println!("Custom function executed!");

            // Set the flag to true when the function is executed
            let mut lock = custom_function_executed_clone.lock().unwrap();
            *lock = true;
        };

        // ThreadWorker 저장
        repository.save_thread_worker("TestWorker", Some(Box::new(custom_function)));

        // start_thread_worker를 호출하여 비동기 작업을 시작
        repository.start_thread_worker("TestWorker");

        // 메인 스레드에서 다른 작업 수행 가능
        for _ in 0..3 {
            println!("Main thread working...");
            sleep(Duration::from_secs(1)).await;
        }

        // 특정 시간 동안 대기하여 비동기 작업이 완료될 때까지 기다립니다.
        sleep(Duration::from_secs(2)).await;

        // 클로저가 실행되었는지 확인하는 코드 추가
        let lock = custom_function_executed.lock().unwrap();
        assert_eq!(*lock, true);
    }

    #[tokio::test]
    async fn test_start_worker_thread_without_custom_function() {
        let custom_function_executed = Arc::new(Mutex::new(false));
        let custom_function_executed_clone = Arc::clone(&custom_function_executed);

        // 테스트를 위한 ThreadWorkerRepositoryImpl 인스턴스 생성
        let repository = ThreadWorkerRepositoryImpl::new();

        // ThreadWorker 저장 (커스텀 함수 없음)
        repository.save_thread_worker("TestWorkerWithoutFunction", None);

        // start_thread_worker를 호출하여 비동기 작업을 시작
        repository.start_thread_worker("TestWorkerWithoutFunction");

        // 메인 스레드에서 다른 작업 수행 가능
        for _ in 0..3 {
            println!("Main thread working...");
            sleep(Duration::from_secs(1)).await;
        }

        // 특정 시간 동안 대기하여 비동기 작업이 완료될 때까지 기다립니다.
        sleep(Duration::from_secs(2)).await;
        println!("2초 대기 완료!");

        // 클로저가 실행되었는지 확인하는 코드 추가 (커스텀 함수가 없으므로 false여야 함)
        let lock = custom_function_executed_clone.lock().unwrap();
        assert_eq!(*lock, false);
    }

    #[tokio::test]
    async fn test_custom_function_execution_without_println() {
        let custom_function_executed = Arc::new(Mutex::new(false));
        let custom_function_executed_clone = Arc::clone(&custom_function_executed);

        // 테스트를 위한 ThreadWorkerRepositoryImpl 인스턴스 생성
        let repository = ThreadWorkerRepositoryImpl::new();

        // 클로저를 따로 변수에 저장 (println! 제거)
        let custom_function = move || {
            // Set the flag to true when the function is executed
            let mut lock = custom_function_executed_clone.lock().unwrap();
            *lock = true;
        };

        // ThreadWorker 저장
        repository.save_thread_worker("TestWorker", Some(Box::new(custom_function)));

        // start_thread_worker를 호출하여 함수 실행
        repository.start_thread_worker("TestWorker");

        // 클로저가 실행되었는지 확인하는 코드 추가
        let lock = custom_function_executed.lock().unwrap();
        assert_eq!(*lock, true);
    }

    #[tokio::test]
    async fn test_custom_function_execution_with_dependency_call() {
        pub struct ClassB;

        impl ClassB {
            pub fn bcall(&self) {
                println!("Class B method called");
            }
        }

        // Define Class A with a dependency on Class B
        pub struct ClassA {
            b_instance: ClassB,
        }

        impl ClassA {
            pub fn new() -> Self {
                ClassA { b_instance: ClassB }
            }

            pub fn acall(&self) {
                println!("Class A method called");
                self.b_instance.bcall();
            }
        }

        let custom_function_executed = Arc::new(Mutex::new(false));
        let custom_function_executed_clone = Arc::clone(&custom_function_executed);
        let repository = ThreadWorkerRepositoryImpl::new();

        let class_a_instance = ClassA::new();

        let custom_function = move || {
            let mut lock = custom_function_executed_clone.lock().unwrap();
            class_a_instance.acall();
            *lock = true;
        };

        repository.save_thread_worker("TestWorker", Some(Box::new(custom_function)));
        repository.start_thread_worker("TestWorker");
        let lock = custom_function_executed.lock().unwrap();
        assert_eq!(*lock, true);
    }
}