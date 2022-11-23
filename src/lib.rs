pub mod client;
pub mod server;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type AsyncVoidResult = AsyncResult<()>;
