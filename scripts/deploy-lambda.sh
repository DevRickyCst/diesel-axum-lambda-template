#!/bin/bash

# ============================================================================
# {{project-name}} - Lambda Deployment Script (SAM)
# Builds Docker image and deploys to AWS Lambda using SAM
# ============================================================================

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
STACK_NAME="${STACK_NAME:-{{project-name}}-prod}"
AWS_REGION="${AWS_REGION:-eu-central-1}"
AWS_PROFILE="${AWS_PROFILE:-default}"
ECR_REPO_NAME="{{project-name}}"

usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Deploy {{project-name}} to AWS Lambda using SAM

OPTIONS:
    -s, --stack-name NAME     CloudFormation stack name [default: {{project-name}}-prod]
    -r, --region REGION       AWS region [default: eu-central-1]
    -p, --profile PROFILE     AWS CLI profile [default: default]
    --create-stack            Create new SAM stack (first deployment)
    --skip-build              Skip Docker build and push
    -h, --help                Show this help message

EXAMPLES:
    # First deployment
    $0 --create-stack

    # Update code
    $0

    # Update with custom profile
    $0 -p production

    # Just update Lambda (skip build)
    $0 --skip-build

PREREQUISITES:
    1. AWS CLI installed and configured
    2. SAM CLI installed (pip install aws-sam-cli)
    3. Docker installed and running
    4. Edit infra/samconfig.toml with your parameters
EOF
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_header() {
    echo ""
    echo -e "${BLUE}============================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}============================================================${NC}"
}

check_dependencies() {
    print_header "Checking Dependencies"

    local missing_deps=()

    if ! command -v docker &> /dev/null; then
        missing_deps+=("docker")
    fi

    if ! command -v aws &> /dev/null; then
        missing_deps+=("aws-cli")
    fi

    if ! command -v sam &> /dev/null; then
        missing_deps+=("sam-cli")
    fi

    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing required dependencies: ${missing_deps[*]}"
        echo ""
        echo "Install AWS SAM CLI:"
        echo "  brew install aws-sam-cli  # macOS"
        echo "  pip install aws-sam-cli    # Python"
        exit 1
    fi

    print_success "All dependencies are installed"
}

get_aws_account_id() {
    aws sts get-caller-identity --profile "$AWS_PROFILE" --query Account --output text
}

get_ecr_repository_uri() {
    aws cloudformation describe-stacks \
        --stack-name "$STACK_NAME" \
        --profile "$AWS_PROFILE" \
        --region "$AWS_REGION" \
        --query 'Stacks[0].Outputs[?OutputKey==`ECRRepositoryUri`].OutputValue' \
        --output text 2>/dev/null || echo ""
}

create_ecr_repository() {
    print_header "Creating ECR Repository"

    local repo_exists=$(aws ecr describe-repositories \
        --repository-names "$ECR_REPO_NAME" \
        --region "$AWS_REGION" \
        --profile "$AWS_PROFILE" \
        --query 'repositories[0].repositoryName' \
        --output text 2>/dev/null || echo "")

    if [ -n "$repo_exists" ]; then
        print_info "ECR repository already exists: $ECR_REPO_NAME"
        return 0
    fi

    print_info "Creating ECR repository: $ECR_REPO_NAME"

    aws ecr create-repository \
        --repository-name "$ECR_REPO_NAME" \
        --region "$AWS_REGION" \
        --profile "$AWS_PROFILE" \
        --image-scanning-configuration scanOnPush=true \
        --output json > /dev/null

    print_success "ECR repository created"
}

login_to_ecr() {
    print_header "Logging in to Amazon ECR"

    local account_id=$(get_aws_account_id)
    print_info "AWS Account ID: $account_id"

    aws ecr get-login-password \
        --region "$AWS_REGION" \
        --profile "$AWS_PROFILE" | \
        docker login \
            --username AWS \
            --password-stdin "$account_id.dkr.ecr.$AWS_REGION.amazonaws.com"

    print_success "Successfully logged in to ECR"
}

build_and_push_image() {
    print_header "Building and Pushing Docker Image"

    cd "$PROJECT_ROOT"

    local account_id=$(get_aws_account_id)
    local ecr_uri="${account_id}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_REPO_NAME}"

    print_info "ECR Repository: $ecr_uri"
    print_info "Building Docker image..."

    docker build \
        --platform linux/amd64 \
        --target runtime \
        -f docker/Dockerfile \
        -t "$ecr_uri:latest" \
        .

    print_success "Docker image built successfully"

    print_info "Pushing image to ECR..."
    docker push "$ecr_uri:latest"

    print_success "Image pushed to ECR successfully"
}

deploy_sam_stack() {
    print_header "Deploying SAM Stack"

    cd "$PROJECT_ROOT/infra"

    print_info "Stack: $STACK_NAME"
    print_info "Region: $AWS_REGION"
    print_info "Profile: $AWS_PROFILE"

    sam deploy \
        --stack-name "$STACK_NAME" \
        --region "$AWS_REGION" \
        --profile "$AWS_PROFILE" \
        --no-confirm-changeset \
        --no-fail-on-empty-changeset

    print_success "SAM stack deployed successfully"
}

show_outputs() {
    print_header "Stack Outputs"

    aws cloudformation describe-stacks \
        --stack-name "$STACK_NAME" \
        --profile "$AWS_PROFILE" \
        --region "$AWS_REGION" \
        --query 'Stacks[0].Outputs[].[OutputKey,OutputValue]' \
        --output table

    local api_url=$(aws cloudformation describe-stacks \
        --stack-name "$STACK_NAME" \
        --profile "$AWS_PROFILE" \
        --region "$AWS_REGION" \
        --query 'Stacks[0].Outputs[?OutputKey==`ApiUrl`].OutputValue' \
        --output text 2>/dev/null)

    if [ -n "$api_url" ]; then
        echo ""
        print_success "API URL: ${api_url}"
        echo ""
        echo "Test endpoints:"
        echo "  curl ${api_url}/health"
        echo "  curl ${api_url}/tasks"
    fi
}

main() {
    local create_stack=false
    local skip_build=false

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -s|--stack-name) STACK_NAME="$2"; shift 2 ;;
            -r|--region) AWS_REGION="$2"; shift 2 ;;
            -p|--profile) AWS_PROFILE="$2"; shift 2 ;;
            --create-stack) create_stack=true; shift ;;
            --skip-build) skip_build=true; shift ;;
            -h|--help) usage; exit 0 ;;
            *) print_error "Unknown option: $1"; usage; exit 1 ;;
        esac
    done

    print_header "{{project-name}} - Lambda Deployment"
    print_info "Stack Name: $STACK_NAME"
    print_info "AWS Region: $AWS_REGION"
    print_info "AWS Profile: $AWS_PROFILE"

    check_dependencies

    if [ "$create_stack" = true ]; then
        # First-time deployment: create ECR, build, push, and deploy
        print_info "Mode: Create Stack (first deployment)"
        create_ecr_repository
        login_to_ecr
        build_and_push_image
        deploy_sam_stack
    else
        # Update deployment: check if stack exists
        local stack_exists=$(aws cloudformation describe-stacks \
            --stack-name "$STACK_NAME" \
            --profile "$AWS_PROFILE" \
            --region "$AWS_REGION" \
            --query 'Stacks[0].StackName' \
            --output text 2>/dev/null || echo "")

        if [ -z "$stack_exists" ]; then
            print_error "Stack '$STACK_NAME' does not exist"
            print_info "Use --create-stack for first deployment"
            exit 1
        fi

        print_info "Mode: Update Stack"

        if [ "$skip_build" = false ]; then
            # Build and push new image
            login_to_ecr
            build_and_push_image
        else
            print_warning "Skipping Docker build and push"
        fi

        # Update Lambda
        deploy_sam_stack
    fi

    show_outputs

    print_header "Deployment Complete"
    print_success "{{project-name}} deployed successfully to AWS Lambda!"
}

main "$@"
