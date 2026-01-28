# Alexa Cookidoo Shopping List Skill

An Alexa Skill built in Rust that enables voice-controlled addition of items to the Thermomix Cookidoo shopping list.

## Prerequisites

- Rust (1.83+)
- [cargo-lambda](https://www.cargo-lambda.info/) for local development and deployment

Install cargo-lambda:
```bash
cargo install cargo-lambda
```

## Project Structure

```
alexa-cookidoo-shopping-list-skill/
├── Cargo.toml              # Workspace configuration
├── .env.example            # Environment variables template
├── skill/                  # Main Lambda function
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs         # Lambda entry point
│   │   ├── lib.rs          # Library root
│   │   ├── domain/         # Core business logic (hexagonal architecture)
│   │   │   ├── models/     # Domain entities (auth, error, shopping_list_item)
│   │   │   ├── ports/      # Interfaces (authentication_service, shopping_list_repository)
│   │   │   └── services/   # Domain services (add_item_service)
│   │   ├── application/    # Application layer
│   │   │   ├── lambda_handler.rs
│   │   │   ├── config.rs
│   │   │   └── dependency_injection.rs
│   │   └── adapters/       # Infrastructure adapters
│   │       ├── alexa/      # Alexa request/response handling
│   │       ├── cookidoo/   # Cookidoo API client
│   │       └── logging/    # Logging setup
│   └── tests/
│       └── fixtures/       # Test fixtures for local development
│           ├── add_item_request.json
│           ├── launch_request.json
│           ├── help_request.json
│           ├── stop_request.json
│           └── ...
└── cdk/                    # Infrastructure (AWS CDK) - coming soon
```

## Local Development

### Start the Lambda emulator

```bash
cargo lambda watch
```

This starts a local server at `http://localhost:9000` that emulates the AWS Lambda runtime. It watches for file changes and automatically recompiles.

### Invoke the function

In another terminal, use the test fixtures to simulate Alexa requests:

```bash
# Test AddItemIntent (adds "Testmilch" to shopping list)
cargo lambda invoke bootstrap --data-file skill/tests/fixtures/add_item_request.json

# Test LaunchRequest
cargo lambda invoke bootstrap --data-file skill/tests/fixtures/launch_request.json

# Test HelpIntent
cargo lambda invoke bootstrap --data-file skill/tests/fixtures/help_request.json

# Test StopIntent
cargo lambda invoke bootstrap --data-file skill/tests/fixtures/stop_request.json
```

Note: For the AddItemIntent to work, you need to configure the Cookidoo credentials in your `.env` file (see `.env.example`).

## Build

### Development build

```bash
cargo build
```

### Release build (optimized for Lambda)

```bash
cargo lambda build --release --arm64
```

## Testing

```bash
cargo test
```