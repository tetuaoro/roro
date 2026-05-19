mod bash;
mod obscura;

pub use bash::*;
pub use obscura::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;
