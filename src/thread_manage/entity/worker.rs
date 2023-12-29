use std::sync::{Arc, Mutex};

struct DebuggableFunction {
    function: Box<dyn Fn() + Send>,
    debug_string: String,
}

impl std::fmt::Debug for DebuggableFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DebuggableFunction")
            .field("debug_string", &self.debug_string)
            .finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub struct Worker {
    name: String,
    custom_function: Option<Arc<Mutex<DebuggableFunction>>>,
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Worker")
            .field("name", &self.name)
            .field(
                "custom_function",
                &self.custom_function
                    .as_ref()
                    .map(|func| func.lock().unwrap().debug_string.clone()),
            )
            .finish()
    }
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && match (&self.custom_function, &other.custom_function) {
            (Some(func1), Some(func2)) => {
                let lock1 = func1.lock().unwrap();
                let lock2 = func2.lock().unwrap();
                lock1.debug_string == lock2.debug_string
            }
            (None, None) => true,
            _ => false,
        }
    }
}

impl Worker {
    pub fn new(name: &str, custom_function: Option<Box<dyn Fn() + Send>>) -> Self {
        let custom_function_with_debug = custom_function.map(|f| {
            let debug_str = format!("CustomFunction({:p})", f.as_ref());
            Arc::new(Mutex::new(DebuggableFunction {
                function: f,
                debug_string: debug_str,
            }))
        });

        Worker {
            name: name.to_string(),
            custom_function: custom_function_with_debug,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // Getter for custom function
    pub fn custom_function(&self) -> Option<Box<dyn Fn() + Send + '_>> {
        self.custom_function
            .as_ref()
            .map(move |func| Box::new(move || (func.lock().unwrap().function)()) as Box<dyn Fn() + Send>)
    }
}

impl AsRef<str> for Worker {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_creation() {
        let worker = Worker::new("John Doe", None);
        assert_eq!(worker.name(), "John Doe");
    }

    #[test]
    fn test_worker_as_ref() {
        let worker = Worker::new("John Doe", None);
        let name_ref: &str = worker.as_ref();
        assert_eq!(name_ref, "John Doe");
    }

    #[test]
    fn test_custom_function() {
        let custom_function = || {
            println!("Custom function executed!");
        };

        let worker = Worker::new("John Doe", Some(Box::new(custom_function.clone())));
        let worker2 = Worker::new("John Doe", Some(Box::new(custom_function)));

        assert_eq!(worker, worker2);
    }

    #[test]
    fn test_get_custom_function() {
        let custom_function = || {
            println!("Custom function executed!");
        };

        let worker = Worker::new("John Doe", Some(Box::new(custom_function.clone())));
        let retrieved_function = worker.custom_function();

        assert_eq!(retrieved_function.is_some(), true);
    }
}
