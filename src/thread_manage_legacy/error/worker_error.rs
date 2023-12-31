use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum WorkerError {
    // Define specific error variants here
    SomeError,
    AnotherError,
    ThreadWriteError,
}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement Display to convert the error to a user-friendly string
        write!(f, "WorkerError: {:?}", self)
    }
}

impl Error for WorkerError {
    // Implement the Error trait here
}