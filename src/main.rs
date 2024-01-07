mod thread_manage_legacy;
mod server_socket;
mod utility;
mod thread_control;
mod client_socket_accept;
mod receiver;


use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::receiver::controller::server_receiver_controller::ServerReceiverController;
use crate::receiver::controller::server_receiver_controller_impl::ServerReceiverControllerImpl;
use crate::receiver::repository::server_receiver_repository::ServerReceiverRepository;
use crate::server_socket::service::server_socket_service::ServerSocketService;
use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;
use crate::thread_control::service::thread_worker_service_impl::ThreadWorkerServiceImpl;
use crate::utility::env::env_detector::EnvDetector;
use crate::utility::initializer::DomainInitializer;
use crate::utility::ip_address::local_ip_finder::IPAddress;

#[tokio::main]
async fn main() {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let domain_initializer = DomainInitializer;
    domain_initializer.init_every_domain().await;

    let server_socket_service = ServerSocketServiceImpl::get_instance();

    let ip = IPAddress::get_local_ip_from_google().unwrap();
    let port = EnvDetector::get_port().unwrap();

    let binding = format!("{}:{}", ip, port).to_string();
    let address = binding.as_str();;

    let mut server_socket_service_guard = server_socket_service.lock().await;

    match server_socket_service_guard.server_socket_bind(address).await {
        Ok(()) => {
            println!("Server bound to address: {}", address);
        }
        Err(err) => {
            eprintln!("Error binding socket: {:?}", err);
        }
    }

    drop(server_socket_service_guard);

    let acceptor_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            let client_socket_accept_controller = ClientSocketAcceptControllerImpl::get_instance();
            let client_socket_accept_controller_guard = client_socket_accept_controller.lock().await;
            println!("Controller instance found. Executing accept_client().");
            client_socket_accept_controller_guard.accept_client().await;
        })
    };

    let thread_worker_service = ThreadWorkerServiceImpl::get_instance();
    let mut thread_worker_service_guard = thread_worker_service.lock().unwrap();

    thread_worker_service_guard.save_async_thread_worker("Acceptor", Box::new(acceptor_function.clone()));
    thread_worker_service_guard.start_thread_worker("Acceptor").await;

    let receiver_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            let server_receiver_controller_mutex = ServerReceiverControllerImpl::get_instance();
            let mut receiver_guard = server_receiver_controller_mutex.lock().await;
            println!("Receiver instance found. Executing client_receive().");
            let _ = receiver_guard.client_receive().await;
        })
    };

    thread_worker_service_guard.save_async_thread_worker("Receiver", Box::new(receiver_function.clone()));
    thread_worker_service_guard.start_thread_worker("Receiver").await;

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
