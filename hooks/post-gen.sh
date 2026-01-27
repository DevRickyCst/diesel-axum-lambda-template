#!/usr/bin/env sh

[ -d .git ] || git init

if [ ! -f .env.example ]; then
  cat > .env.example <<'EOF'
DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db
PORT=3000
EOF
fi

echo "Template ready. Next: cargo build && cargo run"
