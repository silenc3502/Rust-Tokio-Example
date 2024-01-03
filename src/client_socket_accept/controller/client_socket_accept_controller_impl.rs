// use std::sync::{Arc, Mutex, Once};
// use async_trait::async_trait;
// use lazy_static::lazy_static;
// use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
// use crate::client_socket_accept::service::client_socket_accept_service::ClientSocketAcceptService;
// use crate::client_socket_accept::service::client_socket_accept_service_impl::ClientSocketAcceptServiceImpl;
// use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;
//
// #[derive(Clone)]
// pub struct ClientSocketAcceptControllerImpl {
//     service: ClientSocketAcceptServiceImpl,
// }
//
// impl ClientSocketAcceptControllerImpl {
//     pub fn new(service: ClientSocketAcceptServiceImpl) -> Self {
//         ClientSocketAcceptControllerImpl { service }
//     }
//
//     pub fn get_instance() -> &'static Mutex<Option<ClientSocketAcceptControllerImpl>> {
//         INIT.call_once(|| {
//             let service = match ClientSocketAcceptServiceImpl::get_instance() {
//                 Some(service) => service,
//                 None => {
//                     eprintln!("Failed to get ClientSocketAcceptServiceImpl");
//                     return;
//                 }
//             };
//             let controller = ClientSocketAcceptControllerImpl::new(service);
//             *CLIENT_SOCKET_ACCEPT_CONTROLLER.lock().unwrap() = Some(service);
//         });
//         &CLIENT_SOCKET_ACCEPT_CONTROLLER
//     }
// }
//
// #[async_trait]
// impl ClientSocketAcceptController for ClientSocketAcceptControllerImpl {
//     async fn accept_client(&self) {
//         println!("Client Socket Accept Controller: accept()");
//         self.service.accept_client().await;
//     }
// }
//
// lazy_static! {
//     static ref CLIENT_SOCKET_ACCEPT_CONTROLLER: Mutex<Option<ClientSocketAcceptControllerImpl>> = Mutex::new(None);
//     static ref INIT: Once = Once::new();
// }
//
// pub fn get_client_socket_accept_controller() -> &'static Mutex<Option<ClientSocketAcceptControllerImpl>> {
//     INIT.call_once(|| {
//         let service = match ClientSocketAcceptServiceImpl::get_instance() {
//             Some(service) => service,
//             None => {
//                 eprintln!("Failed to get ClientSocketAcceptServiceImpl");
//                 return;
//             }
//         };
//         let controller = ClientSocketAcceptControllerImpl::new(service);
//         *CLIENT_SOCKET_ACCEPT_CONTROLLER.lock().unwrap() = Some(service);
//     });
//     &CLIENT_SOCKET_ACCEPT_CONTROLLER
// }
//
// #[cfg(test)]
// mod tests {
//     use std::any::Any;
//     use super::*;
//     use tokio::time::Duration;
//     use tokio::net::TcpListener;
//     use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ClientSocketAcceptRepositoryImpl;
//     use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;
//     use crate::server_socket::repository::ServerSocketRepositoryImpl::{get_server_socket_repository, ServerSocketRepositoryImpl};
//
//     #[tokio::test]
//     async fn test_controller_singleton_behavior() {
//         // Set up your controller instance
//         let controller1 = ClientSocketAcceptControllerImpl::get_instance();
//         let controller2 = ClientSocketAcceptControllerImpl::get_instance();
//
//         // Ensure that the instances are the same
//         assert!(Arc::ptr_eq(&controller1, &controller2));
//     }
//
//     #[tokio::test]
//     async fn test_controller_accept_client() {
//         get_server_socket_repository();
//         get_client_socket_accept_controller();
//
//         let mut server_socket_repository = ServerSocketRepositoryImpl::get_instance().unwrap();
//         let address = "127.0.0.1:7373";
//         let result = server_socket_repository.bind_socket(address).await;
//
//         assert!(result.is_ok());
//         assert!(server_socket_repository.get_listener().is_some());
//
//         let controller = ClientSocketAcceptControllerImpl::get_instance();
//
//         // Use Tokio's spawn to run the async method
//         tokio::spawn(async move {
//             // Call accept_client on the controller
//             controller.accept_client().await;
//
//             // You may want to add assertions or checks based on the behavior you expect
//         });
//
//         // Sleep for a short duration to allow the async method to run
//         tokio::time::sleep(Duration::from_secs(10)).await;
//
//         // The test passes if it reaches this point without panicking
//     }
//
//     #[tokio::test]
//     async fn test_get_client_socket_accept_controller() {
//         let controller1 = get_client_socket_accept_controller();
//         let controller2 = get_client_socket_accept_controller();
//         assert!(Arc::ptr_eq(&controller1.unwrap(), &controller2.unwrap()));
//     }
// }