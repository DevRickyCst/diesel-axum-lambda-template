# AWS Lambda Deployment Guide

This guide explains how to deploy {{project-name}} to AWS Lambda using AWS SAM (Serverless Application Model).

## Table of Contents

- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Configuration](#configuration)
- [First-Time Deployment](#first-time-deployment)
- [Updating the Deployment](#updating-the-deployment)
- [Monitoring and Logs](#monitoring-and-logs)
- [Useful Commands](#useful-commands)
- [Troubleshooting](#troubleshooting)
- [Cost Estimation](#cost-estimation)
- [Cleanup](#cleanup)

---

## Architecture

The deployment creates the following AWS resources:

```
┌─────────────────────────────────────────────────────────────┐
│                         AWS Cloud                            │
│                                                              │
│  ┌──────────────┐         ┌──────────────┐                 │
│  │ API Gateway  │────────>│   Lambda     │                 │
│  │  (HTTP API)  │         │  Function    │                 │
│  └──────────────┘         └──────────────┘                 │
│         │                        │                          │
│         │                        │                          │
│  ┌──────────────┐         ┌──────────────┐                 │
│  │  CloudWatch  │         │     ECR      │                 │
│  │     Logs     │         │  Repository  │                 │
│  └──────────────┘         └──────────────┘                 │
│                                  │                          │
│                           (Docker Image)                    │
└─────────────────────────────────────────────────────────────┘
```

**Components:**

- **ECR Repository**: Stores the Docker image for the Lambda function
- **Lambda Function**: Runs your Rust application in a containerized environment
- **API Gateway (HTTP API)**: Exposes HTTP endpoints and routes requests to Lambda
- **CloudWatch Logs**: Captures application logs and API Gateway access logs
- **X-Ray**: Distributed tracing for performance monitoring (optional)

---

## Prerequisites

### 1. AWS CLI

Install and configure AWS CLI:

```bash
# macOS
brew install awscli

# Linux
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

# Configure credentials
aws configure
```

### 2. SAM CLI

Install AWS SAM CLI:

```bash
# macOS
brew install aws-sam-cli

# Linux/Windows (via pip)
pip install aws-sam-cli

# Verify installation
sam --version
```

### 3. Docker

Docker must be running to build the Lambda container image:

```bash
# macOS
brew install --cask docker

# Verify
docker --version
docker ps
```

### 4. Required AWS Permissions

Your AWS IAM user/role needs:

- `AmazonEC2ContainerRegistryFullAccess` (ECR)
- `AWSLambda_FullAccess` (Lambda)
- `AmazonAPIGatewayAdministrator` (API Gateway)
- `CloudFormationFullAccess` (CloudFormation)
- `IAMFullAccess` (IAM roles for Lambda)
- `CloudWatchLogsFullAccess` (Logs)

---

## Configuration

### 1. Edit `samconfig.toml`

Update the deployment parameters in `infra/samconfig.toml`:

```toml
[default.deploy.parameters]
stack_name = "{{project-name}}-prod"
region = "eu-central-1"
parameter_overrides = [
    "DatabaseUrl=postgres://user:pass@host:5432/db",  # Your production DB
    "JwtSecret=CHANGE_ME_IN_PRODUCTION",               # Your JWT secret
    "CorsAllowedOrigins=https://yourdomain.com",       # Your frontend URL
    "LambdaMemorySize=1024",                           # Memory in MB
    "LambdaTimeout=30"                                 # Timeout in seconds
]
```

**Important:** Replace placeholder values with your actual production configuration.

### 2. Environment Variables

The Lambda function will have access to:

- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging configuration
- `JWT_SECRET`: (Optional) JWT signing key
- Standard AWS Lambda environment variables

---

## First-Time Deployment

### Step 1: Initial Setup

Navigate to the project root and run:

```bash
./scripts/deploy-lambda.sh --create-stack
```

This will:

1. ✅ Check dependencies (AWS CLI, SAM CLI, Docker)
2. ✅ Create ECR repository
3. ✅ Build Docker image (`linux/amd64`)
4. ✅ Push image to ECR
5. ✅ Deploy CloudFormation stack via SAM

**Duration:** ~10-15 minutes for first deployment

### Step 2: Verify Deployment

After deployment completes, you'll see output like:

```
============================================================
Stack Outputs
============================================================
---------------------------------------------------------
|              ApiUrl                |  https://xxx.execute-api.eu-central-1.amazonaws.com/prod  |
|              FunctionArn           |  arn:aws:lambda:eu-central-1:123456789012:function:...   |
|              ECRRepositoryUri      |  123456789012.dkr.ecr.eu-central-1.amazonaws.com/...     |
---------------------------------------------------------

✅ API URL: https://xxx.execute-api.eu-central-1.amazonaws.com/prod

Test endpoints:
  curl https://xxx.execute-api.eu-central-1.amazonaws.com/prod/health
  curl https://xxx.execute-api.eu-central-1.amazonaws.com/prod/tasks
```

### Step 3: Test the API

```bash
# Health check
curl https://YOUR_API_URL/health

# Expected response:
# {"status":"ok"}
```

---

## Updating the Deployment

### Update Code Only

When you've made code changes and want to update Lambda:

```bash
./scripts/deploy-lambda.sh
```

This will:

1. ✅ Build new Docker image
2. ✅ Push to ECR (overwrites `:latest` tag)
3. ✅ Update Lambda function
4. ✅ CloudFormation detects image change and updates Lambda

**Duration:** ~5-7 minutes

### Update Without Rebuilding

If you only changed SAM template or parameters:

```bash
./scripts/deploy-lambda.sh --skip-build
```

### Custom Configuration

```bash
# Use different AWS profile
./scripts/deploy-lambda.sh --profile production

# Deploy to different region
./scripts/deploy-lambda.sh --region us-east-1

# Use custom stack name
./scripts/deploy-lambda.sh --stack-name my-custom-stack
```

---

## Monitoring and Logs

### View Lambda Logs

```bash
# Follow logs in real-time
sam logs --stack-name {{project-name}}-prod --tail

# View recent logs
aws logs tail /aws/lambda/{{project-name}}-prod --follow

# Filter by time
aws logs tail /aws/lambda/{{project-name}}-prod --since 1h
```

### View API Gateway Logs

```bash
aws logs tail /aws/apigateway/{{project-name}}-prod --follow
```

### CloudWatch Insights

Query logs with CloudWatch Insights:

```sql
fields @timestamp, @message
| filter @message like /ERROR/
| sort @timestamp desc
| limit 20
```

### X-Ray Tracing

View distributed traces in the AWS X-Ray console:

```bash
https://console.aws.amazon.com/xray/home?region=eu-central-1
```

---

## Useful Commands

### SAM Commands

```bash
# Validate template
sam validate --lint

# View stack status
aws cloudformation describe-stacks --stack-name {{project-name}}-prod

# View stack outputs
aws cloudformation describe-stacks \
  --stack-name {{project-name}}-prod \
  --query 'Stacks[0].Outputs'

# View stack resources
aws cloudformation describe-stack-resources \
  --stack-name {{project-name}}-prod
```

### Lambda Commands

```bash
# Invoke function directly
aws lambda invoke \
  --function-name {{project-name}}-prod \
  --payload '{"path": "/health", "httpMethod": "GET"}' \
  response.json

# Get function configuration
aws lambda get-function --function-name {{project-name}}-prod

# Update environment variables only
aws lambda update-function-configuration \
  --function-name {{project-name}}-prod \
  --environment "Variables={DATABASE_URL=new_value}"
```

### ECR Commands

```bash
# List images
aws ecr describe-images --repository-name {{project-name}}

# Delete old images (keep latest 5)
aws ecr list-images --repository-name {{project-name}} \
  --query 'imageIds[5:]' --output json | \
  jq -r '.[] | .imageDigest' | \
  xargs -I {} aws ecr batch-delete-image \
    --repository-name {{project-name}} \
    --image-ids imageDigest={}
```

---

## Troubleshooting

### Issue: "Stack already exists"

**Solution:** Use update mode (don't use `--create-stack`):

```bash
./scripts/deploy-lambda.sh
```

### Issue: Lambda timeout

**Solution:** Increase timeout in `samconfig.toml`:

```toml
"LambdaTimeout=60"  # Increase from 30 to 60 seconds
```

### Issue: Out of memory

**Solution:** Increase memory in `samconfig.toml`:

```toml
"LambdaMemorySize=2048"  # Increase from 1024
```

### Issue: Docker build fails

**Solution:** Ensure Docker is running:

```bash
docker ps
```

If Docker daemon isn't running, start Docker Desktop.

### Issue: ECR authentication fails

**Solution:** Re-authenticate with ECR:

```bash
aws ecr get-login-password --region eu-central-1 | \
  docker login --username AWS --password-stdin \
  $(aws sts get-caller-identity --query Account --output text).dkr.ecr.eu-central-1.amazonaws.com
```

### Issue: CORS errors in browser

**Solution:** Update `CorsAllowedOrigins` in template:

```toml
"CorsAllowedOrigins=https://yourdomain.com,http://localhost:3000"
```

### Issue: Cold start latency

**Solutions:**

1. **Increase memory** (more CPU allocated): `LambdaMemorySize=2048`
2. **Use Provisioned Concurrency** (add to template):
   ```yaml
   ProvisionedConcurrencyConfig:
     ProvisionedConcurrentExecutions: 1
   ```
3. **Implement Lambda warmers** (periodic invocations)

---

## Cost Estimation

### Monthly Costs (Estimate)

Based on **100,000 requests/month** with **512MB RAM** and **1s average duration**:

| Service             | Usage                  | Cost      |
|---------------------|------------------------|-----------|
| Lambda Compute      | 100,000 * 1s * 512MB   | ~$0.83    |
| Lambda Requests     | 100,000 requests       | ~$0.02    |
| API Gateway         | 100,000 requests       | ~$0.10    |
| CloudWatch Logs     | 1 GB logs              | ~$0.50    |
| ECR Storage         | 500 MB images          | ~$0.05    |
| X-Ray (optional)    | 100,000 traces         | ~$0.50    |
| **Total**           |                        | **~$2.00/month** |

**Notes:**

- AWS Free Tier includes 1M Lambda requests/month (first 12 months)
- Costs scale linearly with traffic
- Database hosting (RDS) not included (estimate $15-50/month)

### Cost Optimization Tips

1. **Use appropriate memory size** (don't over-provision)
2. **Set log retention** (30 days instead of forever)
3. **Delete old ECR images** (keep only latest 5)
4. **Monitor with AWS Cost Explorer**

---

## Cleanup

### Delete Everything

To delete all AWS resources created by this deployment:

```bash
# Delete the CloudFormation stack
aws cloudformation delete-stack --stack-name {{project-name}}-prod

# Wait for deletion to complete
aws cloudformation wait stack-delete-complete --stack-name {{project-name}}-prod

# Delete ECR repository
aws ecr delete-repository --repository-name {{project-name}} --force
```

**Warning:** This will permanently delete:

- Lambda function
- API Gateway
- CloudWatch logs
- ECR images

### Partial Cleanup

To keep infrastructure but delete old images:

```bash
# Delete all images except latest
aws ecr list-images --repository-name {{project-name}} \
  --query 'imageIds[1:].[imageDigest]' --output text | \
  xargs -I {} aws ecr batch-delete-image \
    --repository-name {{project-name}} \
    --image-ids imageDigest={}
```

---

## Additional Resources

- [AWS SAM Documentation](https://docs.aws.amazon.com/serverless-application-model/)
- [AWS Lambda Rust Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
- [CloudFormation Template Reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-reference.html)
- [AWS Cost Calculator](https://calculator.aws/)

---

**Need Help?** Check the [main README](../README.md) or open an issue in the repository.
