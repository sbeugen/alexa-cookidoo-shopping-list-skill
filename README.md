# Alexa Cookidoo Shopping List Skill

An Alexa Skill built in Rust that enables voice-controlled addition of items to the Thermomix Cookidoo shopping list.

## Prerequisites

- Rust (1.83+)
- [cargo-lambda](https://www.cargo-lambda.info/) for local development and deployment
- Node.js (20+) and npm for CDK deployment
- AWS CLI configured with appropriate credentials

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
├── cdk/                    # AWS CDK infrastructure
│   ├── bin/cdk-app.ts      # CDK app entry point
│   └── lib/cdk-stack.ts    # Stack definition
└── .github/workflows/      # CI/CD pipelines
    ├── ci.yml              # Build, test, lint on push/PR
    └── deploy.yml          # Manual deployment workflow
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

## Deployment

The infrastructure is managed with AWS CDK using TypeScript.

### Prerequisites

1. Build the Lambda binary:
   ```bash
   cargo lambda build --release --arm64
   ```

2. Set environment variables for Cookidoo credentials:
   ```bash
   export COOKIDOO_EMAIL="your-email@example.com"
   export COOKIDOO_PASSWORD="your-password"
   export COOKIDOO_CLIENT_ID="your-client-id"
   export COOKIDOO_CLIENT_SECRET="your-client-secret"
   ```

3. Install CDK dependencies:
   ```bash
   cd cdk
   npm install
   ```

### Deploy

```bash
cd cdk
npx cdk deploy
```

### Other CDK Commands

```bash
# Preview changes before deploying
npx cdk diff

# Synthesize CloudFormation template
npx cdk synth

# Destroy the stack
npx cdk destroy
```

### CI/CD

The project includes GitHub Actions workflows:

- **CI** (`.github/workflows/ci.yml`): Runs on push to `main` and pull requests. Executes code quality checks, tests, builds, and security audits.
- **Deploy** (`.github/workflows/deploy.yml`): Manual workflow for deploying to AWS. Requires GitHub secrets for AWS credentials and Cookidoo configuration.