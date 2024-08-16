use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use redis::Commands;
use crate::models::session::Session;

pub fn generate_session_id() -> String {   
    // Creates a random number generator that is local to the current thread.
    thread_rng()
        //  This is a distribution that generates random alphanumeric characters (a-z, A-Z, 0-9)
        .sample_iter(&Alphanumeric)
        // Generate 32 characters
        .take(32) 
        // Converts each generated u8 into a char
        .map(char::from)
        // Collects the characters into a String
        .collect()
}

fn store_session(con: &mut redis::Connection, session: &Session) -> redis::RedisResult<()> {
    let session_key = format!("session:{}", session.session_id);
    let serialized_session = serde_json::to_string(session).unwrap()
    
    
    
    
    ;
    con.set(session_key, serialized_session)?;
    Ok(())
}

fn get_session(con: &mut redis::Connection, session_id: &str) -> redis::RedisResult<Option<Session>> {
    let session_key = format!("session:{}", session_id);
    let session_data: Option<String> = con.get(session_key)?;
    match session_data {
        Some(data) => Ok(Some(serde_json::from_str(&data).unwrap())),
        None => Ok(None),
    }
}