pub mod config;
pub mod model;
pub mod queue;
pub mod reddit;
pub mod task;

pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Result<T> = std::result::Result<T, Error>;
