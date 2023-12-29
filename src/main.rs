mod thread_manage;

use std::thread;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex, RwLock};
use redis::Commands;
use crate::thread_manage::service::worker_service::WorkerServiceTrait;
use crate::thread_manage::service::worker_service_impl::get_worker_service;

// use std::thread;
// use lazy_static::lazy::Lazy;
//
// use crate::thread_manage::entity::worker::Worker;
// use crate::thread_manage::repository::worker_repository::WorkerRepositoryTrait;
// use crate::thread_manage::service::worker_service::WorkerServiceTrait;
//
// use lazy_static::lazy_static;
// use crate::thread_manage::repository::worker_repository_impl::WorkerRepositoryImpl;
// use crate::thread_manage::service::worker_service_impl::WorkerServiceImpl;


fn main() {
    let client = redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to connect to Redis");

    let mut connection = client
        .get_connection()
        .expect("Failed to connect to Redis with password");

    let _: () = redis::cmd("AUTH")
        .arg("eddi@123")
        .query(&mut connection)
        .expect("Failed to authenticate to Redis");

    let _: () = redis::cmd("SET")
        .arg("testkey")
        .arg("Hello, EDDI Redis")
        .query(&mut connection)
        .expect("Failed to set key");

    let result: String = redis::cmd("GET")
        .arg("testkey")
        .query(&mut connection)
        .expect("Failed to get key");

    println!("Value: {}", result);

    let worker_service = get_worker_service();

    // 쓰레드 생성 및 ID 가져오기
    let thread_id = worker_service.lock().unwrap().create_thread("Thread 1");
    println!("Created thread with ID: {:?}", thread_id);

    // 생성한 쓰레드 ID로 쓰레드 조회
    if let Some(worker) = worker_service.lock().unwrap().get_thread(thread_id) {
        println!("Thread found: {:?}", worker);
    } else {
        println!("Thread not found with ID: {:?}", thread_id);
    };

    println!("Main Thread ID: {:?}", thread::current().id());

    // 새로운 스레드 생성
    let handle = thread::spawn(|| {
        // 생성된 스레드의 ID 출력
        println!("Spawned Thread ID: {:?}", thread::current().id());
    });

    // 생성된 스레드의 종료 대기
    handle.join().unwrap();
}
