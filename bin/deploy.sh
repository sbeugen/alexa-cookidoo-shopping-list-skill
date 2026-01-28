#!/usr/bin/env bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CDK_DIR="${PROJECT_ROOT}/cdk"

echo -e "${GREEN}=== Alexa Cookidoo Skill Deployment ===${NC}"
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command_exists cargo-lambda; then
    echo -e "${RED}Error: cargo-lambda is not installed.${NC}"
    echo "Install it with: cargo install cargo-lambda"
    exit 1
fi

if ! command_exists npm; then
    echo -e "${RED}Error: npm is not installed.${NC}"
    echo "Please install Node.js and npm"
    exit 1
fi

if ! command_exists aws; then
    echo -e "${RED}Error: AWS CLI is not installed.${NC}"
    echo "Please install and configure the AWS CLI"
    exit 1
fi

echo -e "${GREEN}✓ All prerequisites installed${NC}"
echo ""

# Load .env file if it exists
if [ -f "${PROJECT_ROOT}/.env" ]; then
    echo -e "${YELLOW}Loading environment variables from .env file...${NC}"
    set -a
    source "${PROJECT_ROOT}/.env"
    set +a
    echo -e "${GREEN}✓ Environment variables loaded${NC}"
    echo ""
fi

# Validate required environment variables
echo -e "${YELLOW}Validating environment variables...${NC}"

REQUIRED_VARS=("COOKIDOO_EMAIL" "COOKIDOO_PASSWORD" "COOKIDOO_CLIENT_ID" "COOKIDOO_CLIENT_SECRET")
MISSING_VARS=()

for var in "${REQUIRED_VARS[@]}"; do
    if [ -z "${!var:-}" ]; then
        MISSING_VARS+=("$var")
    fi
done

if [ ${#MISSING_VARS[@]} -ne 0 ]; then
    echo -e "${RED}Error: Missing required environment variables:${NC}"
    for var in "${MISSING_VARS[@]}"; do
        echo "  - $var"
    done
    echo ""
    echo "Please set these variables or create a .env file in the project root."
    exit 1
fi

echo -e "${GREEN}✓ All required environment variables set${NC}"
echo ""

# Build Rust Lambda function
echo -e "${YELLOW}Building Rust Lambda function...${NC}"
cd "${PROJECT_ROOT}/skill"
cargo lambda build --release --arm64
echo -e "${GREEN}✓ Build complete${NC}"
echo ""

# Install CDK dependencies
echo -e "${YELLOW}Installing CDK dependencies...${NC}"
cd "${CDK_DIR}"
npm ci
echo -e "${GREEN}✓ Dependencies installed${NC}"
echo ""

# Bootstrap CDK if needed
echo -e "${YELLOW}Checking CDK bootstrap status...${NC}"
if ! npx cdk bootstrap --show-template > /dev/null 2>&1; then
    echo "Bootstrapping CDK..."
    npx cdk bootstrap
fi
echo -e "${GREEN}✓ CDK bootstrap complete${NC}"
echo ""

# Deploy the stack
echo -e "${YELLOW}Deploying CDK stack...${NC}"
npx cdk deploy --require-approval never

echo ""
echo -e "${GREEN}=== Deployment Complete ===${NC}"
echo ""
echo "Next steps:"
echo "1. Copy the Lambda ARN from the output above"
echo "2. Go to the Alexa Developer Console (https://developer.amazon.com/alexa/console/ask)"
echo "3. Create or update your skill's endpoint with the Lambda ARN"
echo "4. Test your skill in the Alexa simulator"
