#!/usr/bin/env bash
set -euo pipefail

# Initialize git if missing
if [ ! -d .git ]; then
  git init
fi

# Create env example if missing
if [ ! -f .env.example ]; then
  cat > .env.example <<'EOF'
# Postgres
DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db

# HTTP server
PORT=3000
EOF
fi

echo "Template ready. Next: cargo build && cargo run"
