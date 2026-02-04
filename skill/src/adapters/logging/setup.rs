use tracing_subscriber::EnvFilter;

/// Initializes structured logging for AWS Lambda.
///
/// Configuration:
/// - Uses JSON format for CloudWatch Logs Insights compatibility
/// - Reads log level from `RUST_LOG` environment variable (default: `info`)
/// - Flattens event fields for easier querying
/// - Excludes verbose target names for cleaner logs
///
/// # Panics
/// Panics if the subscriber cannot be set (e.g., called more than once).
///
/// # Example
/// ```ignore
/// alexa_cookidoo_skill::adapters::logging::init();
/// tracing::info!("Application started");
/// ```
pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .json()
        .flatten_event(true)
        .with_target(false)
        .with_current_span(false)
        .without_time() // Lambda adds timestamps
        .init();
}

#[cfg(test)]
mod tests {
    // Note: We can't easily test init() as it can only be called once per process.
    // The logging setup is tested implicitly through integration tests.
}
