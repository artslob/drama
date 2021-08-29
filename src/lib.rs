pub mod config;
pub mod reddit;

pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Result<T> = std::result::Result<T, Error>;
