default:
  @just --list 

fmt:
    @cargo +nightly fmt --all

lint:
    @cargo clippy --workspace --all-targets --all-features -- -D warnings

check:
    @cargo check --workspace --all-targets --all-features

test:
    @cargo test --workspace --all-features    

pg-ps:
    @docker compose ps

pg-up:
    @docker compose up -d
    @. ./.env; echo "DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:5432/$POSTGRES_DB" > .env.pg

pg-down:
    @docker compose down

pg-delete:
    @docker compose down -v

pg-logs:
    @docker compose logs -f postgres

sqlx-migrate:
    @export DATABASE_URL="$(cut -d= -f2- .env.pg)"; sqlx migrate run --source crates/rs-server/migrations

sqlx-prepare:
    @export DATABASE_URL="$(cut -d= -f2- .env.pg)"; cargo sqlx prepare --workspace -- --all-targets --all-features
