# Rust-Tokio-Example
It's for Rust Tokio Example

## Test
```rust
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
```