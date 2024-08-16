// Import DbPool type alias
use crate::db_operations::connections::DbPool;
use std::sync::Arc;

// Define the AppState struct with the connection pool
#[derive(Clone)]
pub struct AppState {
    // Using allows for safe concurrent access across multiple threads
    pub db_pool:   Arc<DbPool>,
}