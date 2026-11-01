pub mod hardware;
pub mod software;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
