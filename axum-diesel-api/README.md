# axum-diesel-api

**WASM-compatible API types for axum-diesel-project**

This crate contains the shared request and response DTOs used by the axum-diesel-project backend. It is designed to be imported by frontend applications without pulling in server-only dependencies.

## Features

- ✅ **WASM-compatible** - Builds for `wasm32-unknown-unknown`
- ✅ **Minimal dependencies** - Only `serde`, `uuid`, and `chrono` (all WASM-compatible)
- ✅ **Type-safe** - Share the same DTOs between frontend and backend
- ✅ **Zero server dependencies** - No Axum, Diesel, or Tokio

## Usage

### In Backend (axum-diesel-project)

The backend automatically uses this crate and wraps responses with Axum integration:

```rust
use axum_diesel_api::{CreateTaskRequest, TaskResponse};
use crate::response::AppResponse;

pub async fn create_task(
    Json(payload): Json<CreateTaskRequest>,
) -> Result<AppResponse<TaskResponse>, AppError> {
    let task = service.create_task(payload)?;
    Ok(AppResponse::created(task))
}
```

### In Frontend (WASM/Web Application)

Add this crate to your frontend's `Cargo.toml`:

```toml
[dependencies]
axum-diesel-api = { path = "../axum-diesel-project/axum-diesel-api" }
```

Then use the types directly:

```rust
use axum_diesel_api::{CreateTaskRequest, TaskResponse, AppResponse};

// Create request
let request = CreateTaskRequest {
    title: "My Task".to_string(),
    description: Some("Task description".to_string()),
    completed: false,
};

// Deserialize response
let response: AppResponse<TaskResponse> = serde_json::from_str(&json_string)?;
```

## Building for WASM

```bash
# Install WASM target (if not already installed)
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

## API Types

### Requests

- `CreateTaskRequest` - Create a new task
- `UpdateTaskRequest` - Update an existing task

### Responses

- `TaskResponse` - Task data
- `ErrorResponse` - Error information

### Response Wrapper

- `AppResponse<T>` - Generic response wrapper with status code
- `StatusCode` - HTTP status code enum (WASM-compatible)

## Dependencies

- **serde** (1.0) - Serialization/deserialization
- **serde_json** (1.0) - JSON support
- **uuid** (1.19) - UUID support with `js` feature for WASM
- **chrono** (0.4) - Date/time with `wasmbind` feature for WASM

All dependencies are carefully chosen to be WASM-compatible.

## License

MIT
