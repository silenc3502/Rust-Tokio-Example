mod thread_manage_legacy;
mod server_socket;
mod utility;
mod thread_control;
mod client_socket_accept;


use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::server_socket::service::server_socket_service::ServerSocketService;
use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;
use crate::thread_control::service::thread_worker_service_impl::ThreadWorkerServiceImpl;
use crate::utility::initializer::DomainInitializer;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let domain_initializer = DomainInitializer;
    domain_initializer.init_server_socket_domain();
    domain_initializer.init_thread_control_domain();
    domain_initializer.init_client_socket_accept_domain();

    let server_socket_service = ServerSocketServiceImpl::get_instance();

    let address = "127.0.0.1:7373";

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



    let acceptor_function = || -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async {
            let client_socket_accept_controller = ClientSocketAcceptControllerImpl::get_instance();
            let client_socket_accept_controller_guard = client_socket_accept_controller.lock().await;
            println!("Controller instance found. Executing accept_client().");
            client_socket_accept_controller_guard.accept_client().await;
            println!("accept_client() executed successfully.");
        })
    };



    let thread_worker_service = ThreadWorkerServiceImpl::get_instance();
    let mut thread_worker_service_guard = thread_worker_service.lock().unwrap();

    thread_worker_service_guard.save_async_thread_worker("Acceptor", Arc::new(Mutex::new(acceptor_function)));
    thread_worker_service_guard.start_thread_worker("Acceptor").await;

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }

    // // 쓰레드 생성 및 ID 가져오기
    // let thread_id = worker_service.lock().unwrap().create_thread("Thread 1");
    // println!("Created thread with ID: {:?}", thread_id);
    //
    // // 생성한 쓰레드 ID로 쓰레드 조회
    // if let Some(worker) = worker_service.lock().unwrap().get_thread(thread_id) {
    //     println!("Thread found: {:?}", worker);
    // } else {
    //     println!("Thread not found with ID: {:?}", thread_id);
    // };

    // println!("Main Thread ID: {:?}", thread::current().id());
    //
    // // 새로운 스레드 생성
    // let handle = thread::spawn(|| {
    //     // 생성된 스레드의 ID 출력
    //     println!("Spawned Thread ID: {:?}", thread::current().id());
    // });
    //
    // // 생성된 스레드의 종료 대기
    // handle.join().unwrap();
}
