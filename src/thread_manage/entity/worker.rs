#[derive(Clone, Debug, PartialEq)]
pub struct Worker {
    name: String,
}

impl Worker {
    pub fn new(name: &str) -> Self {
        Worker {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
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
        let worker = Worker::new("John Doe");

        assert_eq!(worker.name(), "John Doe");
    }

    #[test]
    fn test_worker_as_ref() {
        let worker = Worker::new("John Doe");
        let name_ref: &str = worker.as_ref();

        assert_eq!(name_ref, "John Doe");
    }
}