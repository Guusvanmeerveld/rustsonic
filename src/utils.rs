use rand::{distributions::Alphanumeric, Rng};

pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
