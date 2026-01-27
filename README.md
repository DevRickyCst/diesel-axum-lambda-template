# {{project-name}}

{{description}}

## Stack
- Axum, Tokio, Tower HTTP
- Diesel (Postgres + r2d2)
- Tracing
- AWS Lambda runtime via `lambda_http`

## Run
- Env:
  - `DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db`
  - `PORT=3000`
- Local:
  - `cargo run`
- Lambda env:
  - When `AWS_LAMBDA_FUNCTION_NAME` is set, the app uses `lambda_http`.

## Health
- GET http://localhost:3000/health → `{ "status": "ok" }`

## Tests
- `cargo test`
- Optional: Docker Postgres via `docker/docker-compose.test.yml`
