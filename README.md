# {{project-name}}

{{project_description}}

A production-ready Rust web service template built with Axum, Diesel, PostgreSQL, and AWS Lambda support. Demonstrates clean layered architecture with complete CRUD operations.

## Features

- ✅ **Layered Architecture** - Clean separation: Handlers → Services → Repositories
- ✅ **Type-Safe Database** - Diesel ORM with PostgreSQL
- ✅ **Async Runtime** - Tokio for high-performance async I/O
- ✅ **Connection Pooling** - r2d2 with OnceLock for efficient database connections
- ✅ **Error Handling** - Comprehensive error types with HTTP status mapping
- ✅ **Docker Support** - Multi-stage Dockerfile with hot-reload for development
- ✅ **Testing** - Isolated test environment with Docker Compose
- ✅ **AWS Lambda Ready** - Dual runtime support (local HTTP + Lambda)
- ✅ **Database Migrations** - Diesel CLI integration
- ✅ **Structured Logging** - Tracing with configurable log levels

## Prerequisites

- [Docker](https://www.docker.com/) and Docker Compose
- [Rust](https://www.rust-lang.org/) {{rust_version}} (if running locally without Docker)
- [Diesel CLI](https://diesel.rs/guides/getting-started) (optional, installed in Docker)
- [AWS CLI](https://aws.amazon.com/cli/) and [SAM CLI](https://aws.amazon.com/serverless/sam/) (for Lambda deployment)

## Quick Start

1. **Clone the repository**

```bash
git clone <your-repo-url>
cd {{project-name}}
```

2. **Copy environment variables**

```bash
cp .env.example .env
```

3. **Start the development environment**

```bash
make local-detached
```

4. **Run database migrations**

```bash
make migrate
```

5. **Test the API**

```bash
# Health check
curl http://localhost:3000/health

# Create a task
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "My first task", "completed": false}'

# List all tasks
curl http://localhost:3000/tasks
```

## API Endpoints

### Health Check

```bash
GET /health
```

Response:
```json
{
  "status": "ok"
}
```

### Tasks CRUD

#### List all tasks

```bash
GET /tasks
```

Response:
```json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "My task",
    "description": "Task description",
    "completed": false,
    "created_at": "2026-02-02T12:00:00Z",
    "updated_at": "2026-02-02T12:00:00Z"
  }
]
```

#### Get a single task

```bash
GET /tasks/{id}
```

#### Create a task

```bash
POST /tasks
Content-Type: application/json

{
  "title": "New task",
  "description": "Optional description",
  "completed": false
}
```

Response (201 Created):
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "title": "New task",
  "description": "Optional description",
  "completed": false,
  "created_at": "2026-02-02T12:00:00Z",
  "updated_at": "2026-02-02T12:00:00Z"
}
```

#### Update a task

```bash
PUT /tasks/{id}
Content-Type: application/json

{
  "title": "Updated title",
  "completed": true
}
```

#### Delete a task

```bash
DELETE /tasks/{id}
```

Response: 204 No Content

### Error Responses

All errors follow this format:

```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable error message",
  "details": "Optional additional details"
}
```

## Development Commands

```bash
# Start services (foreground)
make local

# Start services (background)
make local-detached

# Stop all services
make stop

# Restart services
make restart

# View logs
make logs              # All services
make logs-app          # App only
make logs-db           # Database only

# Database operations
make migrate           # Run migrations
make revert            # Revert last migration
make db-shell          # Open PostgreSQL shell
make db-reset          # Reset database (WARNING: deletes data)

# Testing
make test              # Run all tests
make test t=test_name  # Run specific test
make test-watch        # Run tests in watch mode
make test-cleanup      # Clean up test containers

# Code quality
make fmt               # Format code
make fmt-check         # Check formatting
make clippy            # Run Clippy linter
make check             # Check compilation
make ci                # Run all CI checks (fmt, clippy, test)

# Cleanup
make clean             # Clean build artifacts
make clean-all         # Clean everything including Docker volumes

# Shell access
make shell             # Open shell in app container
```

## Project Structure

```
.
├── src/
│   ├── api/                    # Public API types
│   │   ├── error.rs           # ErrorResponse DTO
│   │   ├── requests.rs        # Request DTOs
│   │   ├── responses.rs       # Response DTOs
│   │   └── result.rs          # AppResponse<T> pattern
│   ├── db/
│   │   ├── error.rs           # Repository errors
│   │   ├── connection.rs      # Connection pool (OnceLock)
│   │   ├── schema.rs          # Diesel schema (generated)
│   │   ├── models/            # Database models
│   │   │   └── task.rs
│   │   └── repositories/      # Data access layer
│   │       └── task_repository.rs
│   ├── services/              # Business logic layer
│   │   └── task_service.rs
│   ├── handlers/              # HTTP handlers (thin)
│   │   ├── health.rs
│   │   └── task.rs
│   ├── error.rs               # Application errors
│   ├── app.rs                 # Router configuration
│   └── main.rs                # Entrypoint
├── migrations/                # Diesel migrations
│   └── 00000000000001_create_tasks/
│       ├── up.sql
│       └── down.sql
├── docker/
│   └── Dockerfile             # Multi-stage build
├── tests/
│   └── integration/           # Integration tests
├── docker-compose.yml         # Development environment
├── docker-compose.test.yml    # Test environment
├── Cargo.toml
├── .env.example
├── makefile                   # Development commands
├── CLAUDE.md                  # AI assistant guidance
└── README.md
```

## Technologies

- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Diesel](https://diesel.rs/)** - ORM and query builder
- **[PostgreSQL](https://www.postgresql.org/)** - Database
- **[Tower](https://github.com/tower-rs/tower)** - Middleware (tracing, CORS)
- **[Serde](https://serde.rs/)** - Serialization/deserialization
- **[UUID](https://github.com/uuid-rs/uuid)** - Unique identifiers
- **[Chrono](https://github.com/chronotope/chrono)** - Date/time handling
- **[Anyhow](https://github.com/dtolnay/anyhow)** - Error handling
- **[AWS Lambda HTTP](https://github.com/awslabs/aws-lambda-rust-runtime)** - Lambda runtime

## AWS Lambda Deployment

The template supports AWS Lambda deployment out of the box.

### Prerequisites

- AWS CLI configured with appropriate credentials
- SAM CLI installed
- Docker (for building Lambda images)

### Deploy

```bash
# Build and deploy to AWS
make deploy-lambda

# View deployment status
make deploy-status

# View Lambda logs
make deploy-logs
```

The deployment uses AWS SAM with a container image deployment type. See `infra/template.yaml` for infrastructure details.

## Testing

Tests run in an isolated Docker environment with a temporary PostgreSQL database:

```bash
# Run all tests
make test

# Run specific test
make test t=test_create_task

# Run with verbose output
make test t=test_name -- --nocapture

# Watch mode (requires cargo-watch)
make test-watch
```

The test database uses tmpfs for faster execution (50-70% speed improvement).

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`make test`)
4. Run code quality checks (`make ci`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

[MIT License](LICENSE)

## Author

{{author_name}} <{{author_email}}>

---

**Note:** This is a template generated with [cargo-generate](https://github.com/cargo-generate/cargo-generate). Remove this note and customize the README for your project.
