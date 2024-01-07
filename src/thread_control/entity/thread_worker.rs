use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::thread_control::entity::closure::Closure;
use crate::thread_control::entity::custom_future::CustomFuture;

pub struct ThreadWorker {
    name: String,
    will_be_execute_function: Option<Closure>,
}

impl ThreadWorker {
    pub fn new(name: &str) -> Self {
        ThreadWorker {
            name: name.to_string(),
            will_be_execute_function: None,
        }
    }

    pub fn save_function(&mut self, f: Closure) {
        self.will_be_execute_function = Some(f);
    }

    pub fn get_will_be_execute_function_ref(&self) -> Option<&Closure> {
        self.will_be_execute_function.as_ref()
    }

    pub fn get_will_be_execute_function(&mut self) -> Option<CustomFuture> {
        println!("ThreadWorker return closure");
        if let Some(closure) = self.will_be_execute_function.take() {
            match closure {
                Closure::Async(mut f) => Some(CustomFuture(Box::pin(f()))),
                Closure::Sync(mut f) => {
                    f();
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AsRef<str> for ThreadWorker {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

// impl PartialEq for ThreadWorker {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//             && match (&self.will_be_execute_function, &other.will_be_execute_function) {
//             (Some(Closure::Async(f1)), Some(Closure::Async(f2))) => {
//                 let guard1 = tokio::runtime::Runtime::new()
//                     .unwrap()
//                     .block_on(timeout(Duration::from_secs(5), (*f1).lock()));
//                 let guard2 = tokio::runtime::Runtime::new()
//                     .unwrap()
//                     .block_on(timeout(Duration::from_secs(5), (*f2).lock()));
//
//                 match (guard1, guard2) {
//                     (Ok(guard1), Ok(guard2)) => std::ptr::eq(&*guard1, &*guard2),
//                     _ => false,
//                 }
//             }
//             (None, None) => true,
//             _ => false,
//         }
//     }
// }



impl fmt::Debug for ThreadWorker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadWorker")
            .field("name", &self.name)
            .field(
                "will_be_execute_function",
                &match &self.will_be_execute_function {
                    Some(_) => "Some(Arc<Mutex<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>>)",
                    None => "None",
                },
            )
            .finish()
    }
}

unsafe impl Send for ThreadWorker {}

unsafe impl Sync for ThreadWorker {}

#[cfg(test)]
mod tests {
    use tokio::spawn;
    use super::*;

    fn my_sync_function() {
        println!("Synchronous function is executed!");
    }

    async fn my_async_function() {
        println!("Asynchronous function is executed!");
    }

    #[tokio::test]
    async fn test_custom_function() {
        let custom_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        let mut worker = ThreadWorker::new("EDDI-LV2");
        worker.save_function(Closure::Async(Box::new(custom_function.clone())));

        let found_function_name = worker.name();
        println!("check found_function_name name: {}", found_function_name);

        let closure_option = worker.get_will_be_execute_function();
        assert_eq!(closure_option.is_some(), true);

        if let Some(closure) = closure_option {
            println!("found closure!");
            closure.0.await;
        }else {
            println!("No custom function found!");
        }
    }

    #[tokio::test]
    async fn test_get_will_be_execute_function() {
        let custom_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        let mut worker = ThreadWorker::new("EDDI");
        worker.save_function(Closure::Async(Box::new(custom_function.clone())));

        let found_function_name = worker.name();
        println!("check found_function_name name: {}", found_function_name);

        let closure_option = worker.get_will_be_execute_function();
        assert_eq!(closure_option.is_some(), true);

        if let Some(closure) = closure_option {
            println!("found closure!");
            closure.0.await;
        }else {
            println!("No custom function found!");
        }
    }

    #[tokio::test]
    async fn test_my_sync_function() {
        let custom_function = || {
            my_sync_function();
        };

        let mut worker = ThreadWorker::new("EDDI RobotAcademy");
        worker.save_function(Closure::Sync(Box::new(custom_function)));

        let found_function_name = worker.name();
        println!("check found_function_name name: {}", found_function_name);

        worker.get_will_be_execute_function();
    }

    #[tokio::test]
    async fn test_my_async_function() {
        let custom_function = || {
            Box::pin(my_async_function()) as Pin<Box<dyn Future<Output = ()> + Send>>
        };

        let mut worker = ThreadWorker::new("Revolution");
        worker.save_function(Closure::Async(Box::new(custom_function.clone())));

        let found_function_name = worker.name();
        println!("check found_function_name name: {}", found_function_name);

        let closure_option = worker.get_will_be_execute_function();
        assert_eq!(closure_option.is_some(), true);

        if let Some(closure) = worker.get_will_be_execute_function() {
            println!("found closure!");
            spawn(closure.0);
        }else {
            println!("No custom function found!");
        }
    }
}
