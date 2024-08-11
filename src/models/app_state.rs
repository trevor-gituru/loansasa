// Import DbPool type alias
use crate::db_operations::db::DbPool;

// Define the AppState struct with the connection pool
#[derive(Clone)]
pub struct AppState {
    pub pool:   DbPool,
}