# Alexa Cookidoo Shopping List Skill - Implementation Plan

## Project Overview

An Alexa Skill built in Rust that enables voice-controlled addition of items to the Thermomix Cookidoo shopping list. The skill uses hexagonal architecture to maintain clean separation of concerns and testability.

**Target Language**: Rust
**Target Locale**: de-DE (Germany)
**Architecture Pattern**: Hexagonal Architecture (Ports & Adapters)
**Deployment**: AWS Lambda
**Infrastructure**: AWS CDK (Rust) *(not yet implemented)*
**API Base URL**: `https://de.tmmobile.vorwerk-digital.com`

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Technology Stack](#technology-stack)
3. [Project Structure](#project-structure)
4. [Module Specifications](#module-specifications)
5. [Infrastructure Setup](#infrastructure-setup)
6. [Testing Strategy](#testing-strategy)
7. [CI/CD Pipeline](#cicd-pipeline)
8. [Implementation Phases](#implementation-phases)
9. [Configuration & Deployment](#configuration--deployment)
10. [Monitoring & Logging](#monitoring--logging)

---

## Architecture Overview

### Hexagonal Architecture Pattern

The application follows the **Hexagonal Architecture** (Ports & Adapters) pattern to achieve:
- Clear separation of business logic from external concerns
- High testability through dependency injection
- Easy swapping of adapters (e.g., testing with mock Cookidoo API)
- Independent evolution of layers

```
┌─────────────────────────────────────────────────────────────┐
│                    ADAPTERS LAYER (Outer)                   │
│                                                             │
│  ┌────────────────┐                    ┌─────────────────┐  │
│  │ Alexa Adapter  │                    │ Cookidoo Adapter│  │
│  │                │                    │                 │  │
│  │ • Request      │                    │ • HTTP Client   │  │
│  │   Parsing      │                    │ • Auth Handler  │  │
│  │ • Response     │                    │ • Token Cache   │  │
│  │   Building     │                    │ • API Calls     │  │
│  └────────┬───────┘                    └────────┬────────┘  │
│           │                                     │           │
│           │  ┌───────────────────────────────┐  │           │
│           └─▶│   APPLICATION LAYER           │◀-┘           │
│              │                               │              │
│              │  • Dependency Injection       │              │
│              │  • Lambda Handler             │              │
│              │  • Configuration              │              │
│              │  • Orchestration              │              │
│              └─────────────┬─────────────────┘              │
│                            │                                │
│              ┌─────────────▼─────────────────┐              │
│              │      DOMAIN LAYER (Core)      │              │
│              │                               │              │
│              │  • Models (Entities)          │              │
│              │  • Ports (Trait Definitions)  │              │
│              │  • Business Logic             │              │
│              │  • Domain Services            │              │
│              │                               │              │
│              │  ⚠️ NO external dependencies  │              │
│              └───────────────────────────────┘              │
└─────────────────────────────────────────────────────────────┘
```

### Request Flow

```
User Voice Command
      ↓
Alexa Service
      ↓
[AWS Lambda] ← Entry Point
      ↓
Lambda Handler (Application Layer)
      ↓
Alexa Adapter → Parse Request
      ↓
Domain Service (Add Item Service)
      ↓
Cookidoo Adapter → Check Token Cache
      ↓
[Token Valid?]
   ↓ No         ↓ Yes
Authenticate   Use Cached Token
      ↓              ↓
   Cache Token ←─────┘
      ↓
Add Item API Call
      ↓
Response Builder
      ↓
Alexa Response JSON
      ↓
User Hears Confirmation
```

---

## Technology Stack

### Core Rust Crates

| Category | Crate | Version | Purpose |
|----------|-------|---------|---------|
| **Async Runtime** | `tokio` | 1.x | Async execution |
| **HTTP Client** | `reqwest` | 0.13.1 | HTTP requests to Cookidoo |
| **Serialization** | `serde` | 1.x | JSON ser/de |
| | `serde_json` | 1.0.149 | JSON handling |
| **Error Handling** | `thiserror` | 2.0.18 | Custom error types |
| | `anyhow` | 1.x | Error context |
| **Lambda** | `lambda_runtime` | 1.0.2 | Lambda runtime |
| **Logging** | `tracing` | 0.1.x | Structured logging |
| | `tracing-subscriber` | 0.3.x | Log formatting (with json, env-filter) |
| **Async Traits** | `async-trait` | 0.1.x | Async trait methods |
| **Environment** | `dotenvy` | 0.15 | Environment variable loading |
| **Testing** | `mockall` | 0.14.0 | Mocking framework |
| | `wiremock` | 0.6.5 | HTTP mocking |

### AWS Services Required

- **AWS Lambda**: Function execution
- **Amazon Alexa Skills Kit**: Skill interface
- **AWS CloudWatch**: Logging and monitoring
- **AWS IAM**: Permissions management

### Development Tools

- **Cargo Lambda**: Build and deploy tool for Rust Lambda functions
- **AWS CDK**: Infrastructure as code
- **GitHub Actions**: CI/CD automation

---

## Project Structure

```
alexa-cookidoo-skill/
├── README.md                           # Project documentation
├── PROJECT.md                          # Implementation plan (this file)
├── .gitignore                          # Git ignore patterns
├── .env.example                        # Example environment variables
├── Cargo.toml                          # Workspace configuration
├── Cargo.lock                          # Dependency lock file
│
├── skill/                              # Main application (Rust project)
│   ├── Cargo.toml                      # Application dependencies
│   ├── src/
│   │   ├── lib.rs                      # Library root
│   │   ├── main.rs                     # Binary entry point (Lambda)
│   │   │
│   │   ├── domain.rs                   # ═══ DOMAIN LAYER ═══ (module declaration)
│   │   ├── domain/
│   │   │   ├── models.rs               # Domain Models (declares submodules)
│   │   │   ├── models/
│   │   │   │   ├── shopping_list_item.rs    # Item entity + unit tests
│   │   │   │   ├── auth.rs                  # Auth entities + unit tests
│   │   │   │   └── error.rs                 # Domain errors
│   │   │   │
│   │   │   ├── ports.rs                # Trait Definitions (declares submodules)
│   │   │   ├── ports/
│   │   │   │   ├── shopping_list_repository.rs
│   │   │   │   └── authentication_service.rs
│   │   │   │
│   │   │   ├── services.rs             # Business Logic (declares submodules)
│   │   │   └── services/
│   │   │       └── add_item_service.rs # Core use case + unit tests
│   │   │
│   │   ├── adapters.rs                 # ═══ ADAPTERS LAYER ═══ (module declaration)
│   │   ├── adapters/
│   │   │   ├── cookidoo.rs             # Cookidoo API Adapter (declares submodules)
│   │   │   ├── cookidoo/
│   │   │   │   ├── client.rs           # HTTP client wrapper + unit tests
│   │   │   │   ├── auth.rs             # Auth implementation
│   │   │   │   ├── shopping_list.rs    # Shopping list operations
│   │   │   │   ├── models.rs           # API request/response models + unit tests
│   │   │   │   ├── error.rs            # Cookidoo-specific errors
│   │   │   │   └── token_cache.rs      # Token caching logic + unit tests
│   │   │   │
│   │   │   ├── alexa.rs                # Alexa Skill Adapter (declares submodules)
│   │   │   ├── alexa/
│   │   │   │   ├── handler.rs          # Main Alexa handler + unit tests
│   │   │   │   ├── models.rs           # Alexa JSON models + unit tests
│   │   │   │   ├── intent_parser.rs    # Intent parsing logic + unit tests
│   │   │   │   └── response_builder.rs # Response construction + unit tests
│   │   │   │
│   │   │   ├── logging.rs              # Logging Adapter (declares submodules)
│   │   │   └── logging/
│   │   │       └── setup.rs            # Logging initialization + unit tests
│   │   │
│   │   ├── application.rs              # ═══ APPLICATION LAYER ═══ (module declaration)
│   │   └── application/
│   │       ├── lambda_handler.rs       # Lambda entry point + unit tests
│   │       ├── config.rs               # Configuration management + unit tests
│   │       └── dependency_injection.rs # DI container
│   │
│   └── tests/                          # Integration Tests
│       ├── alexa_integration.rs        # Alexa handler integration tests
│       ├── cookidoo_integration.rs     # Cookidoo adapter integration tests
│       │
│       └── fixtures/                   # Test data
│           ├── add_item_request.json           # AddItem intent request
│           ├── add_item_empty_slot_request.json # AddItem with empty slot
│           ├── help_request.json               # Help intent request
│           ├── launch_request.json             # Launch request
│           ├── stop_request.json               # Stop intent request
│           ├── cookidoo_auth_success.json      # Auth success response
│           ├── cookidoo_auth_error.json        # Auth error response
│           └── cookidoo_add_item_success.json  # Add item success response
│
├── cdk/                               # ═══ INFRASTRUCTURE ═══ (not yet implemented)
│   └── ...                            # AWS CDK stack definitions
│
└── .github/                           # ═══ CI/CD ═══ (not yet implemented)
    └── workflows/
        └── ...                        # GitHub Actions workflows
```

---

## Module Specifications

### 1. Domain Layer

The core of the application containing pure business logic with zero external dependencies.

#### 1.1 Domain Models (`src/domain/models/`)

**`shopping_list_item.rs`**
- **Entity**: `ShoppingListItem`
- **Responsibilities**:
    - Represent a shopping list item
    - Validate item name (non-empty, max length)
    - Provide immutable access to item properties
- **Validation Rules**:
    - Name cannot be empty
    - Name max length: 200 characters
    - Trim whitespace

**`auth.rs`**
- **Entities**:
    - `CookidooCredentials`: Email + password
    - `AuthToken`: Access token, refresh token, expiry time
- **Responsibilities**:
    - Store authentication data
    - Check token expiration
    - Determine if token needs refresh (5-minute buffer)
- **Key Methods**:
    - `is_expired() -> bool`
    - `needs_refresh() -> bool`
    - `access_token() -> &str`

**`error.rs`**
- **Error Types**:
    - `InvalidItemName`: Validation failures
    - `AuthenticationFailed`: Auth errors
    - `RepositoryError`: Generic repository failures
- **Implementation**: Use `thiserror` crate for derive macros

#### 1.2 Ports (`src/domain/ports/`)

Trait definitions that specify contracts for external systems.

**`shopping_list_repository.rs`**
```rust
#[async_trait]
pub trait ShoppingListRepository: Send + Sync {
    async fn add_item(&self, item: &ShoppingListItem) -> Result<(), DomainError>;
}
```
- **Purpose**: Define contract for shopping list operations
- **Thread-Safety**: Must be `Send + Sync` for Lambda concurrency

**`authentication_service.rs`**
```rust
#[async_trait]
pub trait AuthenticationService: Send + Sync {
    async fn authenticate(
        &self,
        credentials: &CookidooCredentials,
    ) -> Result<AuthToken, DomainError>;
    
    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken, DomainError>;
}
```
- **Purpose**: Define contract for authentication operations
- **Methods**: Login and token refresh

#### 1.3 Domain Services (`src/domain/services/`)

**`add_item_service.rs`**
- **Purpose**: Orchestrate the "add item" use case
- **Dependencies**: Accepts any `ShoppingListRepository` implementation
- **Logic**:
    1. Validate and create `ShoppingListItem` from raw input
    2. Call repository to add item
    3. Return user-friendly success/error message
- **Error Handling**: Convert domain errors to user messages
- **Logging**: Log at info level for success, error level for failures
- **Testing**: Use mocked repository for unit tests

---

### 2. Adapters Layer

Implementations of domain ports for specific technologies.

#### 2.1 Cookidoo Adapter (`src/adapters/cookidoo/`)

**`models.rs`**
- **Purpose**: Define API request/response structures
- **Models**:
    - `CookidooAuthResponse`: Token response from auth endpoint
    - `AddItemRequest`: Request body for adding item
    - `IngredientRequest`: Nested ingredient structure
    - `AddItemResponse`: Response after adding item
- **Serialization**: Use `serde` for JSON conversion

**`error.rs`**
- **Error Types**:
    - `RequestError`: Network/HTTP failures
    - `AuthenticationError`: Login failures (401, 400)
    - `ParseError`: JSON parsing failures
    - `HttpError`: General HTTP errors with status code
    - `TokenExpired`: Token refresh failures
- **Implementation**: Use `thiserror` for structured errors

**`token_cache.rs`**
- **Purpose**: Thread-safe in-memory token cache
- **Implementation**:
    - Use `Arc<RwLock<Option<AuthToken>>>`
    - Allows multiple readers, single writer
    - Survives across Lambda invocations (warm starts)
- **Methods**:
    - `get() -> Option<AuthToken>`
    - `set(token: AuthToken)`
    - `clear()`
    - `is_valid() -> bool`
- **Thread-Safety**: Critical for Lambda concurrency model

**`auth.rs`**
- **Purpose**: Implement `AuthenticationService` port
- **API Details**:
    - Base URL: `https://de.tmmobile.vorwerk-digital.com`
    - Auth Endpoint: `/ciam/auth/token`
    - Method: POST (form-encoded)
    - Login params:
        - `grant_type`: "password"
        - `username`: email
        - `password`: password
        - `client_id`: configured client ID
    - Refresh params:
        - `grant_type`: "refresh_token"
        - `refresh_token`: token
- **Token Management**:
    1. Check cache for valid token
    2. If expired but refresh token available, refresh
    3. If refresh fails or no token, perform full login
    4. Cache successful tokens
- **Error Handling**:
    - 401: Invalid credentials
    - 400: Bad request format
    - Handle timeouts and connection errors
- **Logging**: Debug level for requests, error level for failures

**`shopping_list.rs`**
- **Purpose**: Implement `ShoppingListRepository` port
- **API Details**:
    - Base URL: `https://de.tmmobile.vorwerk-digital.com`
    - Endpoint: `/shopping/de-DE/additional-items/add`
    - Method: POST
    - Headers:
        - `Authorization: Bearer {access_token}`
        - `Content-Type: application/json`
    - Body: See `AddItemRequest` model
- **Flow**:
    1. Get valid token from auth adapter
    2. Make POST request with item
    3. Handle 401 (re-auth may be needed)
    4. Return success/failure
- **Dependencies**: Holds reference to `CookidooAuthAdapter`

**`client.rs`**
- **Purpose**: Wrapper around `reqwest::Client`
- **Configuration**:
    - Connection pooling
    - Timeout settings (30 seconds recommended)
    - User-Agent header
    - Retry logic (optional)

#### 2.2 Alexa Adapter (`src/adapters/alexa/`)

**`models.rs`**
- **Purpose**: Define Alexa JSON request/response structures
- **Request Models**:
    - `AlexaRequest`: Top-level request wrapper
    - `Session`: Session information
    - `Application`: Skill application ID
    - `User`: User ID
    - `Request`: Enum for different request types
        - `LaunchRequest`
        - `IntentRequest`
        - `SessionEndedRequest`
    - `Intent`: Intent name and slots
    - `Slot`: Slot name and value
- **Response Models**:
    - `AlexaResponse`: Top-level response
    - `ResponseBody`: Output speech, cards, etc.
    - `OutputSpeech`: Enum for PlainText or SSML
- **Serialization**: Use `serde` with `rename_all = "camelCase"`

**`intent_parser.rs`**
- **Purpose**: Parse Alexa requests into domain-friendly intents
- **Output Enum**: `ParsedIntent`
    - `AddItem { item_name: String }`
    - `Help`
    - `Cancel`
    - `Stop`
    - `Launch`
    - `Unknown`
- **Logic**:
    - Match on `Request` type
    - Extract intent name and slots
    - For `AddItemIntent`, extract "Item" slot value
    - Handle missing or empty slots gracefully
- **Error Handling**: Return `Unknown` for unparseable intents

**`response_builder.rs`**
- **Purpose**: Build Alexa response JSON
- **Methods**:
    - `success(message)`: Success response, end session
    - `error(message)`: Error response, end session
    - `help()`: Help text, keep session open
    - `launch()`: Welcome message, keep session open
    - `goodbye()`: Farewell message, end session
- **Speech Format**: Use PlainText for simplicity
- **Session Management**: Control `shouldEndSession` flag

**`handler.rs`**
- **Purpose**: Main Alexa skill handler
- **Dependencies**: Takes `AddItemService<R>` where `R: ShoppingListRepository`
- **Flow**:
    1. Parse `AlexaRequest` using `IntentParser`
    2. Match on `ParsedIntent`
    3. For `AddItem`, call domain service
    4. Build appropriate response
    5. Serialize to JSON
- **Error Handling**:
    - Catch domain errors
    - Return user-friendly Alexa responses
    - Never expose internal errors to users
- **Logging**: Log all requests and responses at info level

#### 2.3 Logging Adapter (`src/adapters/logging/`)

**`setup.rs`**
- **Purpose**: Initialize structured logging for Lambda
- **Configuration**:
    - Use `tracing-subscriber` with JSON formatter
    - Read log level from `RUST_LOG` env var (default: `info`)
    - Include thread IDs for debugging
    - Exclude target module names for cleaner logs
- **Lambda Integration**: JSON output compatible with CloudWatch Logs Insights

---

### 3. Application Layer

Orchestrates the application, wires dependencies, and handles Lambda-specific concerns.

**`config.rs`**
- **Purpose**: Application configuration
- **Config Struct**: `AppConfig`
    - `cookidoo_credentials: CookidooCredentials`
- **Loading**: `from_env()` method
    - Read `COOKIDOO_EMAIL` from environment
    - Read `COOKIDOO_PASSWORD` from environment
    - Return error if missing
- **Security**: Never log credentials

**`dependency_injection.rs`**
- **Purpose**: Wire all dependencies together
- **Pattern**: Builder/Factory pattern
- **Flow**:
    1. Create `CookidooAuthAdapter` with credentials
    2. Create `CookidooShoppingListAdapter` with auth adapter
    3. Create `AddItemService` with repository
    4. Create `AlexaSkillHandler` with service
    5. Return fully wired handler
- **Singleton**: Create once at Lambda cold start

**`lambda_handler.rs`**
- **Purpose**: Lambda function entry point
- **Implementation**:
    - Use `lambda_runtime::run` with `service_fn`
    - Parse incoming event as `AlexaRequest`
    - Call `AlexaSkillHandler.handle()`
    - Return JSON response
- **Initialization**:
    - Call logging setup once
    - Load config once
    - Wire dependencies once
    - Reuse across warm invocations
- **Error Handling**:
    - Catch all errors at top level
    - Return generic error response to Alexa
    - Log full error details
- **Performance**: Minimize work in handler, maximize reuse

**`main.rs`**
- **Purpose**: Binary entry point
- **Implementation**:
    - Initialize logging
    - Load configuration
    - Wire dependencies
    - Start Lambda runtime
    - Handle shutdown signals

---

## Infrastructure Setup

### CDK Project Structure (`cdk/`)

**`src/main.rs`**
- **Purpose**: CDK app entry point
- **Implementation**:
    - Create CDK App
    - Instantiate `AlexaSkillStack`
    - Synthesize CloudFormation template
- **Configuration**: Read from `cdk.json`

**`src/stacks/alexa_skill_stack.rs`**
- **Purpose**: Define all AWS resources
- **Resources**:
    1. **Lambda Function**:
        - Runtime: Custom Runtime (Rust binary)
        - Handler: N/A (use custom runtime)
        - Memory: 256 MB (adjust based on testing)
        - Timeout: 30 seconds
        - Architecture: arm64 (for cost savings)
        - Environment Variables:
            - `COOKIDOO_EMAIL` (from CDK context/secrets)
            - `COOKIDOO_PASSWORD` (from CDK context/secrets)
            - `RUST_LOG=info`
    2. **IAM Role**:
        - Basic Lambda execution role
        - CloudWatch Logs permissions
    3. **Lambda Permission**:
        - Allow Alexa Skills Kit to invoke function
        - Principal: `alexa-appkit.amazon.com`
        - Condition: Alexa Skill ID
    4. **CloudWatch Log Group**:
        - Retention: 7 days (configurable)
        - Log format: JSON
- **Outputs**:
    - Lambda Function ARN (for Alexa Skill configuration)
    - Log Group name

**`cdk.json`**
- **Purpose**: CDK configuration
- **Contents**:
    - App command: `cargo run`
    - Context parameters
    - Feature flags
    - Build settings

**`bin/deploy.sh`**
- **Purpose**: Deployment script
- **Steps**:
    1. Build Rust binary with Cargo Lambda
    2. Bootstrap CDK (if needed)
    3. Deploy stack
    4. Output Lambda ARN
- **Requirements**:
    - `cargo-lambda` installed
    - AWS credentials configured
    - CDK CLI installed

**`bin/destroy.sh`**
- **Purpose**: Cleanup script
- **Steps**:
    1. Destroy CDK stack
    2. Clean up bootstrap resources (optional)

### Alexa Skill Configuration

**Skill Manifest** (created in Alexa Developer Console):
- **Invocation Name**: "cookidoo einkaufsliste" (or similar German phrase)
- **Endpoint**: AWS Lambda ARN (from CDK output)
- **Intents**:
    1. **AddItemIntent**:
        - Utterances:
            - "füge {Item} hinzu"
            - "schreibe {Item} auf die Liste"
            - "ich brauche {Item}"
            - "{Item} auf die Einkaufsliste"
        - Slots:
            - `Item` (type: AMAZON.Food, AMAZON.Product)
    2. **Built-in Intents**:
        - `AMAZON.HelpIntent`
        - `AMAZON.CancelIntent`
        - `AMAZON.StopIntent`
- **Locale**: de-DE
- **Permissions**: None required (no account linking)

---

## Testing Strategy

### 1. Unit Tests (inline in source files)

Unit tests are located within the source files using `#[cfg(test)]` modules. This keeps tests close to the code they test.

**Files with unit tests**:
- `domain/models/shopping_list_item.rs` - Item validation tests
- `domain/models/auth.rs` - Auth token expiry logic tests
- `domain/services/add_item_service.rs` - Service tests with mocked repository
- `adapters/cookidoo/client.rs` - HTTP client wrapper tests
- `adapters/cookidoo/models.rs` - API model serialization tests
- `adapters/cookidoo/token_cache.rs` - Token caching tests
- `adapters/alexa/handler.rs` - Handler tests with mocked service
- `adapters/alexa/models.rs` - Alexa JSON model tests
- `adapters/alexa/intent_parser.rs` - Intent parsing tests
- `adapters/alexa/response_builder.rs` - Response builder tests
- `adapters/logging/setup.rs` - Logging setup tests
- `application/config.rs` - Configuration loading tests
- `application/lambda_handler.rs` - Lambda handler tests

**Framework**: Standard Rust `#[test]` and `tokio::test` for async
**Mocking**: `mockall` for trait mocking

### 2. Integration Tests (`skill/tests/`)

Integration tests are in the `tests/` directory and test component interactions.

**`cookidoo_integration.rs`**
- **Test Coverage**:
    - Authentication flow
    - Token caching
    - Token refresh
    - Add item API call
    - Error scenarios (401, 400, timeouts)
- **Framework**: `wiremock` for HTTP mocking
- **Setup**:
    - Start mock Cookidoo server
    - Configure adapters to use mock URL
    - Test real HTTP interactions without hitting prod API
- **Scenarios**:
    - Successful login
    - Invalid credentials
    - Expired token refresh
    - Network failures

**`alexa_integration.rs`**
- **Test Coverage**:
    - Full request/response flow
    - Intent handling end-to-end
- **Framework**: Standard integration tests
- **Fixtures**: Load JSON from `fixtures/` directory
- **Approach**:
    - Test complete Alexa handler flow
    - Mock Cookidoo adapter

### 3. Test Fixtures (`skill/tests/fixtures/`)

**Alexa request fixtures**:
- `launch_request.json` - Launch request
- `add_item_request.json` - AddItem intent with valid slot
- `add_item_empty_slot_request.json` - AddItem intent with empty slot
- `help_request.json` - Help intent
- `stop_request.json` - Stop intent

**Cookidoo response fixtures**:
- `cookidoo_auth_success.json` - Successful auth response
- `cookidoo_auth_error.json` - Auth error response
- `cookidoo_add_item_success.json` - Add item success response

### 4. Test Execution

**Commands**:
```bash
# Run all tests (unit + integration)
cargo test

# Run unit tests only (tests in lib.rs and src/)
cargo test --lib

# Run integration tests only
cargo test --test '*'

# Run specific integration test file
cargo test --test cookidoo_integration
cargo test --test alexa_integration

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_add_item_success

# Coverage (with tarpaulin)
cargo tarpaulin --out Html
```

**Coverage Goals**:
- Domain layer: 90%+ coverage
- Adapters: 80%+ coverage
- Application layer: 70%+ coverage

---

## CI/CD Pipeline

### GitHub Actions Workflows

#### 1. CI Pipeline (`.github/workflows/ci.yml`)

**Triggers**:
- Push to `main` branch

**Jobs**:

**Job: Check**
- Steps:
    1. Checkout code
    2. Install Rust toolchain
    3. Run `cargo check`
    4. Run `cargo clippy` (fail on warnings)
    5. Run `cargo fmt --check`

**Job: Test**
- Steps:
    1. Checkout code
    2. Install Rust toolchain
    3. Run `cargo test --all-features`
    4. Upload test results

**Job: Build**
- Steps:
    1. Checkout code
    2. Install Cargo Lambda
    3. Build release binary: `cargo lambda build --release --arm64`
    4. Verify binary size
    5. Upload artifact

**Job: Security**
- Steps:
    1. Checkout code
    2. Run `cargo audit` (check for vulnerabilities)
    3. Run `cargo deny` (check licenses)

**Matrix Strategy**: Test on multiple Rust versions (stable, beta)

#### 2. Deployment Pipeline (`.github/workflows/deploy.yml`)

**Triggers**:
- Manual workflow dispatch

**Jobs**:

**Job: Deploy**
- Steps:
    1. Checkout code
    2. Configure AWS credentials (use GitHub secrets)
    3. Install Rust toolchain
    4. Install Cargo Lambda
    5. Install CDK CLI
    6. Build Lambda binary
    7. Deploy CDK stack
    8. Output Lambda ARN
    9. Comment ARN on commit (if PR)

**Environment**:
- Use GitHub Environments for approval gates (optional)
- Store secrets:
    - `AWS_ACCESS_KEY_ID`
    - `AWS_SECRET_ACCESS_KEY`
    - `COOKIDOO_EMAIL`
    - `COOKIDOO_PASSWORD`

**Rollback Strategy**:
- CDK automatically maintains previous versions
- Manual rollback: `cdk deploy --rollback`

#### 3. Security Scanning (`.github/workflows/security.yml`)

**Triggers**:
- Schedule: Weekly
- Manual workflow dispatch

**Jobs**:

**Job: Dependency Audit**
- Run `cargo audit`
- Report vulnerabilities as issues

**Job: SAST Scanning**
- Use tools like `cargo-geiger` for unsafe code detection
- Report findings

---

## Implementation Phases

### Phase 1: Project Setup & Domain Layer (Week 1)

**Goals**:
- Set up project structure
- Implement domain layer
- Write unit tests for domain

**Tasks**:
1. Initialize Cargo workspace
2. Create `skill/` project with dependencies
3. Implement domain models:
    - `ShoppingListItem`
    - `AuthToken`
    - `CookidooCredentials`
    - Domain errors
4. Define ports (traits)
5. Implement `AddItemService`
6. Write unit tests
7. Set up logging adapter
8. Document domain layer

**Deliverables**:
- Fully tested domain layer
- 90%+ test coverage
- Documentation

### Phase 2: Cookidoo Adapter (Week 2)

**Goals**:
- Implement Cookidoo API client
- Handle authentication and token caching
- Write integration tests

**Tasks**:
1. Define Cookidoo models
2. Implement `CookidooAuthAdapter`:
    - Login flow
    - Token refresh
    - Token caching
3. Implement `CookidooShoppingListAdapter`:
    - Add item API call
    - Error handling
4. Write integration tests with `wiremock`
5. Test against real Cookidoo API (manual)
6. Document API endpoints and flows

**Deliverables**:
- Working Cookidoo adapter
- Integration tests
- API documentation

### Phase 3: Alexa Adapter (Week 3)

**Goals**:
- Implement Alexa skill handler
- Parse intents and build responses
- Write integration tests

**Tasks**:
1. Define Alexa JSON models
2. Implement `IntentParser`
3. Implement `ResponseBuilder`
4. Implement `AlexaSkillHandler`
5. Write integration tests with sample requests
6. Test with Alexa Simulator (manual)
7. Document intent structure

**Deliverables**:
- Working Alexa adapter
- Integration tests
- Alexa interaction model documentation

### Phase 4: Application Layer & Lambda Integration (Week 4)

**Goals**:
- Wire dependencies
- Create Lambda handler
- Local testing

**Tasks**:
1. Implement configuration loading
2. Implement dependency injection
3. Implement Lambda handler
4. Local testing with `cargo lambda watch`
5. Test with sample Alexa events
6. Optimize performance and cold start time
7. Document deployment process

**Deliverables**:
- Working Lambda function (local)
- End-to-end tests
- Performance benchmarks

### Phase 5: Infrastructure & Deployment (Week 5)

**Goals**:
- Create CDK stack
- Deploy to AWS
- Configure Alexa Skill

**Tasks**:
1. Create CDK project
2. Define Lambda function resource
3. Define IAM roles and permissions
4. Create deployment scripts
5. Deploy to AWS
6. Configure Alexa Skill in Developer Console
7. Link Lambda ARN to skill
8. Test with real Alexa device
9. Document infrastructure

**Deliverables**:
- Deployed Lambda function
- Configured Alexa Skill
- Infrastructure documentation

### Phase 6: CI/CD & Documentation (Week 6)

**Goals**:
- Set up CI/CD pipelines
- Complete documentation
- Final testing

**Tasks**:
1. Create CI workflow
2. Create deployment workflow
3. Set up GitHub secrets
4. Write README.md
5. Write CONTRIBUTING.md
6. Write deployment guide
7. Perform security audit
8. End-to-end testing
9. Performance tuning
10. Create demo video (optional)

**Deliverables**:
- Full CI/CD pipeline
- Complete documentation
- Production-ready system

---

## Configuration & Deployment

### Environment Variables

**Required**:
- `COOKIDOO_EMAIL`: Cookidoo account email
- `COOKIDOO_PASSWORD`: Cookidoo account password

**Optional**:
- `RUST_LOG`: Log level (default: `info`)
- `COOKIDOO_API_URL`: Override API URL for testing (default: production)

### Secrets Management

**During Development**:
- Store in `.env` file (git-ignored)
- Use `dotenv` crate for local testing

**During Deployment**:
- Store in GitHub Secrets for CI/CD
- Pass to CDK via context or environment
- CDK sets as Lambda environment variables

**Security Considerations**:
- Never commit credentials to git
- Use AWS Secrets Manager for production (optional enhancement)
- Rotate credentials regularly
- Use IAM roles where possible

### Deployment Steps

**Prerequisites**:
1. AWS account with appropriate permissions
2. AWS CLI configured
3. Rust toolchain installed
4. Cargo Lambda installed: `cargo install cargo-lambda`
5. CDK CLI installed: `npm install -g aws-cdk`
6. Alexa Developer account

**Initial Deployment**:

```bash
# 1. Clone repository
git clone <repository-url>
cd alexa-cookidoo-skill

# 2. Build Lambda function
cd skill
cargo lambda build --release --arm64

# 3. Deploy infrastructure
cd ../cdk
export COOKIDOO_EMAIL="your-email@example.com"
export COOKIDOO_PASSWORD="your-password"
./bin/deploy.sh

# 4. Note the Lambda ARN from output
# ARN: arn:aws:lambda:eu-central-1:123456789:function:AlexaSkillFunction

# 5. Configure Alexa Skill
# - Go to Alexa Developer Console
# - Create new skill
# - Set endpoint to Lambda ARN
# - Configure interaction model
# - Test in simulator

# 6. Test with device
# "Alexa, öffne Cookidoo Einkaufsliste"
# "Füge Milch hinzu"
```

**Subsequent Deployments**:

```bash
# Update code
git pull origin main

# Rebuild and redeploy
cd skill && cargo lambda build --release --arm64
cd ../cdk && cdk deploy
```

**Automated Deployment** (via CI/CD):
- Push to `main` branch
- GitHub Actions builds and deploys automatically
- Monitor deployment in Actions tab

### Rollback Procedure

**If deployment fails**:
```bash
cd cdk
cdk deploy --rollback
```

**If Lambda has issues**:
- CloudWatch Logs provide error details
- Previous Lambda version remains available
- Revert to previous commit and redeploy

---

## Monitoring & Logging

### CloudWatch Logs

**Log Groups**:
- `/aws/lambda/AlexaSkillFunction`

**Log Format**: JSON (structured logging)

**Log Levels**:
- `ERROR`: Critical failures
- `WARN`: Recoverable errors
- `INFO`: Normal operations (default)
- `DEBUG`: Detailed debugging (for development)

**Key Log Fields**:
- `timestamp`: ISO 8601 timestamp
- `level`: Log level
- `message`: Log message
- `thread_id`: Thread identifier
- `fields`: Additional structured data

**Example Log Entry**:
```json
{
  "timestamp": "2024-01-27T10:30:15.123Z",
  "level": "INFO",
  "message": "Adding item to shopping list: Milch",
  "fields": {
    "item_name": "Milch"
  }
}
```

### CloudWatch Metrics

**Custom Metrics** (optional enhancement):
- `AddItemSuccess`: Counter
- `AddItemFailure`: Counter
- `CookidooAuthFailure`: Counter
- `ColdStartDuration`: Timer
- `RequestDuration`: Timer

**Lambda Metrics** (automatic):
- Invocations
- Errors
- Duration
- Throttles
- Concurrent executions

### CloudWatch Alarms

**Recommended Alarms**:
1. **High Error Rate**:
    - Metric: Lambda Errors
    - Threshold: > 5 in 5 minutes
    - Action: SNS notification

2. **High Duration**:
    - Metric: Lambda Duration
    - Threshold: > 10 seconds
    - Action: SNS notification

3. **Throttling**:
    - Metric: Lambda Throttles
    - Threshold: > 0
    - Action: SNS notification

### Debugging

**Local Testing**:
```bash
# Run with debug logging
RUST_LOG=debug cargo lambda watch

# Invoke with test event
cargo lambda invoke --data-file tests/fixtures/alexa_requests.json
```

**Production Debugging**:
1. Check CloudWatch Logs for error messages
2. Search for request ID in logs
3. Trace request through all log entries
4. Check Cookidoo API responses
5. Verify token caching behavior

**Common Issues**:
- **401 Unauthorized**: Check credentials, token may have expired
- **Timeout**: Increase Lambda timeout, optimize code
- **Cold Start**: Optimize binary size, consider provisioned concurrency
- **Alexa Not Responding**: Check Lambda permissions, verify ARN

---

## Additional Considerations

### Performance Optimization

**Binary Size Reduction**:
- Use `opt-level = "z"` in release profile
- Strip symbols: `strip = true`
- Avoid large dependencies
- Use `cargo-bloat` to analyze binary size

**Cold Start Optimization**:
- Minimize initialization work
- Use lazy static for expensive resources
- Consider Provisioned Concurrency for consistent performance
- Target: < 500ms cold start

**Runtime Optimization**:
- Reuse HTTP connections
- Cache tokens across invocations
- Use efficient serialization
- Target: < 2 seconds total response time

### Security Best Practices

**Credentials**:
- Never log credentials
- Use environment variables
- Consider AWS Secrets Manager for production
- Rotate credentials regularly

**Code Security**:
- Run `cargo audit` regularly
- Keep dependencies updated
- Minimize use of `unsafe` code
- Use `cargo deny` for license compliance

**Lambda Security**:
- Principle of least privilege for IAM role
- Enable VPC if accessing private resources
- Use AWS WAF for API Gateway (if added)
- Monitor for unusual activity

### Scalability

**Lambda Concurrency**:
- Default: 1000 concurrent executions
- Reserve concurrency if needed
- Monitor throttling metrics

**Cookidoo API**:
- Respect rate limits (unknown, monitor)
- Implement backoff/retry logic
- Handle 429 responses gracefully

**Cost Optimization**:
- Use Arm64 architecture (20% cheaper)
- Right-size memory allocation
- Monitor costs with AWS Cost Explorer
- Consider free tier limits

### Future Enhancements

**Features**:
1. Support multiple Cookidoo accounts (account linking)
2. Remove items from list
3. Read shopping list
4. Add multiple items in one request
5. Support for quantities and units
6. Integration with Alexa Shopping List

**Technical**:
1. Add DynamoDB for state persistence
2. Implement circuit breaker for Cookidoo API
3. Add metrics and tracing (OpenTelemetry)
4. Implement request/response caching
5. Add API Gateway for HTTP endpoint
6. Create web interface for configuration

**Operations**:
1. Set up alerting and monitoring
2. Create runbooks for common issues
3. Implement automated testing in production
4. Add performance benchmarking
5. Create dashboard for metrics

---

## Glossary

**Terms**:
- **Hexagonal Architecture**: Architectural pattern separating core logic from external concerns
- **Port**: Interface (trait) defining contract for external system
- **Adapter**: Implementation of port for specific technology
- **Domain Layer**: Core business logic with no external dependencies
- **Cold Start**: First invocation of Lambda function instance
- **Warm Start**: Reuse of existing Lambda function instance
- **Intent**: User's goal in Alexa interaction
- **Slot**: Parameter in Alexa intent (e.g., item name)
- **Utterance**: Example phrase user might say
- **Session**: Conversation context in Alexa

---

## References

- **Rust Lambda Runtime**: https://github.com/awslabs/aws-lambda-rust-runtime
- **Cargo Lambda**: https://www.cargo-lambda.info/
- **AWS CDK**: https://docs.aws.amazon.com/cdk/
- **Alexa Skills Kit**: https://developer.amazon.com/alexa/alexa-skills-kit
- **Hexagonal Architecture**: https://alistair.cockburn.us/hexagonal-architecture/
- **Rust Async Book**: https://rust-lang.github.io/async-book/

---

## Appendix

### A. Sample Alexa Skill Manifest

```json
{
  "manifest": {
    "publishingInformation": {
      "locales": {
        "de-DE": {
          "name": "Cookidoo Einkaufsliste",
          "summary": "Füge Artikel zur Cookidoo Einkaufsliste hinzu",
          "description": "Mit diesem Skill kannst du per Sprache Artikel zu deiner Cookidoo Einkaufsliste hinzufügen.",
          "examplePhrases": [
            "Alexa, öffne Cookidoo Einkaufsliste",
            "Alexa, füge Milch zu Cookidoo hinzu",
            "Alexa, sage Cookidoo dass ich Eier brauche"
          ],
          "keywords": ["cookidoo", "thermomix", "einkaufsliste", "shopping"]
        }
      }
    },
    "apis": {
      "custom": {
        "endpoint": {
          "uri": "arn:aws:lambda:eu-central-1:123456789:function:AlexaSkillFunction"
        }
      }
    },
    "manifestVersion": "1.0"
  }
}
```

### B. Sample Interaction Model (de-DE)

```json
{
  "interactionModel": {
    "languageModel": {
      "invocationName": "cookidoo einkaufsliste",
      "intents": [
        {
          "name": "AddItemIntent",
          "slots": [
            {
              "name": "Item",
              "type": "AMAZON.Food",
              "samples": [
                "{Item}"
              ]
            }
          ],
          "samples": [
            "füge {Item} hinzu",
            "schreibe {Item} auf die Liste",
            "ich brauche {Item}",
            "{Item} auf die Einkaufsliste",
            "notiere {Item}",
            "füge {Item} zur Liste hinzu"
          ]
        },
        {
          "name": "AMAZON.HelpIntent",
          "samples": []
        },
        {
          "name": "AMAZON.CancelIntent",
          "samples": []
        },
        {
          "name": "AMAZON.StopIntent",
          "samples": []
        }
      ]
    }
  }
}
```

### C. Cargo.toml (Current)

**Workspace (`Cargo.toml`)**:
```toml
[workspace]
members = ["skill"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1.0.149"
thiserror = "2.0.18"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

**Skill (`skill/Cargo.toml`)**:
```toml
[package]
name = "alexa-cookidoo-skill"
version.workspace = true
edition.workspace = true

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }

# Lambda
lambda_runtime = "1.0.2"

# HTTP
reqwest = { version = "0.13.1", default-features = false, features = ["json", "rustls", "form"] }

# Async
async-trait = "0.1"

# Environment
dotenvy = "0.15"

[dev-dependencies]
tokio = { workspace = true, features = ["test-util"] }
wiremock = "0.6.5"
mockall = "0.14.0"

[[bin]]
name = "bootstrap"
path = "src/main.rs"
```

### D. CDK Dependencies (not yet implemented)

The CDK infrastructure is planned but not yet implemented. When implemented, it will be added as a workspace member.

---

## Summary

This implementation plan provides a comprehensive roadmap for building an Alexa Skill in Rust that adds items to the Cookidoo shopping list. The architecture follows hexagonal principles for maintainability and testability, with clear separation between domain logic, adapters, and application concerns.

**Key Design Decisions**:
1. **Hexagonal Architecture**: Ensures testability and flexibility
2. **Token Caching**: Optimizes performance across Lambda invocations
3. **Arm64 Architecture**: Reduces costs and improves performance
4. **Structured Logging**: Facilitates debugging and monitoring
5. **Comprehensive Testing**: Unit, integration, and E2E tests
6. **Automated CI/CD**: GitHub Actions for quality and deployment

**Estimated Timeline**: 6 weeks for full implementation with testing and documentation

**Success Criteria**:
- ✅ User can add items via voice command
- ✅ < 2 second response time
- ✅ 90%+ domain layer test coverage
- ✅ Production-ready error handling
- ✅ Automated deployment pipeline
- ✅ Comprehensive documentation