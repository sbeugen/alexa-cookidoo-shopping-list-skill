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
├── Cargo.toml          # Workspace configuration
├── skill/              # Main Lambda function
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs     # Lambda entry point
│       └── lib.rs      # Library root
└── cdk/                # Infrastructure (AWS CDK) - coming soon
```

## Local Development

### Start the Lambda emulator

```bash
cargo lambda watch
```

This starts a local server at `http://localhost:9000` that emulates the AWS Lambda runtime. It watches for file changes and automatically recompiles.

### Invoke the function

In another terminal:

```bash
cargo lambda invoke bootstrap --data-ascii '{}'
```

Or with a custom event:

```bash
cargo lambda invoke bootstrap --data-ascii '{"key": "value"}'
```

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