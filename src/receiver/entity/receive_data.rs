pub struct ReceiveData {
    receive_content: Vec<u8>,
}

impl ReceiveData {
    pub fn new() -> Self {
        ReceiveData {
            receive_content: Vec::new(),
        }
    }

    pub fn get_receive_content(&self) -> &[u8] {
        &self.receive_content
    }

    pub fn receive_content_mut(&mut self) -> &mut Vec<u8> {
        &mut self.receive_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_data() {
        let mut receive_data = ReceiveData::new();

        receive_data.receive_content_mut().extend_from_slice(b"Hello, World!");

        let stored_data = receive_data.get_receive_content();
        assert_eq!(stored_data, b"Hello, World!" as &[u8]);
    }
}
