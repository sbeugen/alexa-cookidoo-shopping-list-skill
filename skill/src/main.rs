use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use tracing::{error, info};

use alexa_cookidoo_skill::adapters::logging;
use alexa_cookidoo_skill::application::{handle_request, AppConfig, Container};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load .env file if present (for local development)
    let _ = dotenvy::from_filename("../.env");

    // Initialize logging first
    logging::init();

    info!("Lambda cold start - initializing");

    // Load configuration
    let config = match AppConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            error!(error = %e, "Failed to load configuration");
            return Err(e.into());
        }
    };

    // Wire dependencies (done once at cold start)
    let container = Container::new(config);

    info!("Initialization complete, starting Lambda runtime");

    // Run the Lambda runtime
    lambda_runtime::run(service_fn(|event: LambdaEvent<Value>| async {
        handle_request(event, container.handler()).await
    }))
    .await
}
