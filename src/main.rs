mod thread_manage;

use std::thread;

use redis::Commands;
use crate::thread_manage::service::worker_service::WorkerServiceTrait;
use crate::thread_manage::service::worker_service_impl::WorkerServiceImpl;
// use crate::thread_manage::service::worker_service_impl::get_worker_service;


fn main() {
    // let client = redis::Client::open("redis://127.0.0.1/")
    //     .expect("Failed to connect to Redis");
    //
    // let mut connection = client
    //     .get_connection()
    //     .expect("Failed to connect to Redis with password");
    //
    // let _: () = redis::cmd("AUTH")
    //     .arg("eddi@123")
    //     .query(&mut connection)
    //     .expect("Failed to authenticate to Redis");
    //
    // let _: () = redis::cmd("SET")
    //     .arg("testkey")
    //     .arg("Hello, EDDI Redis")
    //     .query(&mut connection)
    //     .expect("Failed to set key");
    //
    // let result: String = redis::cmd("GET")
    //     .arg("testkey")
    //     .query(&mut connection)
    //     .expect("Failed to get key");
    //
    // println!("Value: {}", result);

    let worker_service = WorkerServiceImpl::get_instance();

    worker_service.lock().unwrap().as_ref().unwrap().create_thread("Alice", None);

    // Retrieve the worker named "Alice"
    let retrieved_worker = worker_service
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .get_thread("Alice");

    // Display information about the retrieved worker
    match retrieved_worker {
        Some(worker) => {
            println!("Retrieved worker: {:?}", worker);
            // Accessing the name using the custom function getter
            match worker.custom_function() {
                Some(custom_function) => {
                    println!("Executing custom function:");
                    custom_function();
                }
                None => {
                    println!("No custom function available for this worker.");
                }
            }
        }
        None => {
            println!("Worker not found.");
        }
    }

    let custom_function = || {
        println!("Custom function executed!");
    };

    worker_service
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .create_thread("Tester", Some(Box::new(custom_function)));

    // Retrieve the worker named "Alice"
    let retrieved_worker = worker_service
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .get_thread("Tester");

    // Display information about the retrieved worker
    match retrieved_worker {
        Some(worker) => {
            println!("Retrieved worker: {:?}", worker);
            // Accessing the name using the custom function getter
            match worker.custom_function() {
                Some(custom_function) => {
                    println!("Executing custom function:");
                    custom_function();
                }
                None => {
                    println!("No custom function available for this worker.");
                }
            }
        }
        None => {
            println!("Worker not found.");
        }
    }
    //
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
    //
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
