mod thread_manage_legacy;
mod server_socket;
mod utility;
mod thread_control;
mod client_socket_accept;


use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
// use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::server_socket::service::ServerSocketService::ServerSocketService;
use crate::server_socket::service::ServerSocketServiceImpl::ServerSocketServiceImpl;
use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;
// use redis::Commands;
// use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;
use crate::thread_control::service::thread_worker_service_impl::ThreadWorkerServiceImpl;
// use crate::thread_manage_legacy::service::worker_service::WorkerServiceTrait;
// use crate::thread_manage_legacy::service::worker_service_impl::WorkerServiceImpl;
use crate::utility::initializer::DomainInitializer;
// use crate::thread_manage_legacy::service::worker_service_impl::get_worker_service;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let domain_initializer = DomainInitializer;
    domain_initializer.init_server_socket_domain();
    domain_initializer.init_thread_control_domain();
    domain_initializer.init_client_socket_accept_domain();

    let server_socket_service = ServerSocketServiceImpl::get_instance();

    // Define the address to bind to
    let address = "127.0.0.1:7373";

    // Lock the mutex to get a mutable reference to ServerSocketServiceImpl
    let mut service_guard = server_socket_service.lock().unwrap();

    // Dereference the MutexGuard to access the inner ServerSocketServiceImpl
    let server_socket_service_impl = service_guard.as_mut().expect("Service not initialized");

    // Call the bind method on the inner ServerSocketServiceImpl
    match server_socket_service_impl.bind(address).await {
        Ok(()) => {
            println!("Server bound to address: {}", address);
        }
        Err(err) => {
            eprintln!("Error binding socket: {:?}", err);
        }
    }

    let client_socket_accept_controller = ClientSocketAcceptControllerImpl::get_instance();

    let thread_worker_service = ThreadWorkerServiceImpl::get_instance();

    let custom_function = move || {
        println!("Controller instance found. Executing accept_client().");
        // Attempt to execute accept_client() on the controller
        client_socket_accept_controller.accept_client();
        println!("accept_client() executed successfully.");
    };

    // Create a thread with the custom function
    thread_worker_service
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .create_thread("Acceptor", Some(Box::new(custom_function)));

    // Start the worker thread
    thread_worker_service
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .start_worker("Acceptor");

    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

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
