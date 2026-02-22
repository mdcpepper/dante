set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

ext_so := "target/debug/liblattice.so"

default: test

cloc:
    cloc fixtures crates php --exclude_dir vendor,dist

test-rust:
    cargo test --all-features --workspace --exclude lattice-php-ext

build-extension:
    cd crates/php-ext && cargo build

test-extension: build-extension
    cd php && php -d extension=../{{ ext_so }} vendor/bin/pest --configuration=phpunit.xml

test: test-rust test-extension

dev:
    docker compose up -d --wait postgres
    docker compose --profile dev up --build --force-recreate json-api-dev demo

remove:
    docker compose --profile dev down --volumes --remove-orphans --rmi local

sqlx *args='':
    #!/usr/bin/env bash
    set -euo pipefail
    db_url="${DATABASE_ADMIN_URL_DOCKER:-postgresql://${POSTGRES_USER:-lattice_user}:${POSTGRES_PASSWORD:-lattice_password}@postgres:5432/${POSTGRES_DB:-lattice_db}}"
    docker compose --profile dev run --rm --build --quiet-build -T -e DATABASE_URL="$db_url" json-api-dev sqlx "$@"

migrate:
    just sqlx migrate run

cli *args='':
    #!/usr/bin/env bash
    set -euo pipefail
    db_url="${DATABASE_ADMIN_URL:-postgresql://${POSTGRES_USER:-lattice_user}:${POSTGRES_PASSWORD:-lattice_password}@localhost:5432/${POSTGRES_DB:-lattice_db}}"
    DATABASE_URL="$db_url" cargo run --package lattice-app -- "$@"
