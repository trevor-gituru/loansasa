use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

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