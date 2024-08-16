use redis::{Commands, Client, Connection};
use dotenv::dotenv;
use std::env;


fn connect_redis() -> Connection {
    // Load environment variables from .env file
    dotenv().ok();
    // Get the REDIS URL from environment variables
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");


    // Create a Redis client with the specified URL
    let client = Client::open(&*redis_url).expect("Invalid Redis URL");

    // Get a connection from the client
    client.get_connection().expect("Failed to connect to Redis")
}

fn main() {
    let mut con = connect_redis();

    // Test Redis connection
    let _: () = con.set("test_key", "test_value").expect("Failed to set key");
    let result: String = con.get("test_key").expect("Failed to get key");

    println!("The value for 'test_key' is: {}", result);
}