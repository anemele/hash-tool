mod cli;
mod core;

pub use cli::cli;
pub use core::parse;
pub use core::sha256::hash_file_sha256;
pub use core::HashLine;
