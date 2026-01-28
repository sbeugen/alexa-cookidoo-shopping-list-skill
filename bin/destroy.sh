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

echo -e "${YELLOW}=== Destroying Alexa Cookidoo Skill Stack ===${NC}"
echo ""

# Check if CDK is available
if [ ! -d "${CDK_DIR}/node_modules" ]; then
    echo -e "${YELLOW}Installing CDK dependencies...${NC}"
    cd "${CDK_DIR}"
    npm ci
fi

# Destroy the stack
cd "${CDK_DIR}"
echo -e "${YELLOW}Destroying CDK stack...${NC}"
npx cdk destroy --force

echo ""
echo -e "${GREEN}=== Stack Destroyed ===${NC}"
echo ""
echo "Note: Remember to also remove the Lambda ARN from your Alexa skill configuration."
