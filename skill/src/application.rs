mod config;
mod dependency_injection;
mod lambda_handler;

pub use config::AppConfig;
pub use dependency_injection::Container;
pub use lambda_handler::handle_request;