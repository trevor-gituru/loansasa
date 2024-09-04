use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use deadpool_redis::Config;


use std::env;

// Define a type alias for the connection pool
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type RedisPool = deadpool_redis::Pool;


// Function to create the database pool
pub fn establish_db_connection() -> DbPool {
    // Load environment variables from .env file
    dotenv().ok(); 
    // Get the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create the connection manager
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Build the connection pool
    r2d2::Pool::builder()
        .max_size(15) // Set the maximum number of connections in the pool
        .build(manager)
        .expect("Failed to create pool.")
}

// Function to create the Redis connection pool
pub fn establish_redis_connection() -> RedisPool {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the Redis URL from environment variables
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // Create the Redis connection pool
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool().unwrap();
    return pool;
}