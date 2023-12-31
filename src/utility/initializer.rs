use crate::server_socket::service::ServerSocketServiceImpl::get_server_socket_service;

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_server_socket_domain(&self) {
        let _ = get_server_socket_service();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_socket_service_initialization() {
        let domain_initializer = DomainInitializer;
        domain_initializer.init_server_socket_domain();

        let initialized_service = get_server_socket_service();
        assert!(initialized_service.lock().unwrap().is_some());
    }
}
