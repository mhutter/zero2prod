#!/usr/bin/env bash
set -e -u -o pipefail

DOCKER="${DOCKER:=podman}"

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker
if [ -z "${SKIP_DOCKER:=}" ]; then
  "$DOCKER" run -d --rm --name postgres_newsletter \
    -p "127.0.0.1:${DB_PORT}:5432" \
    -e "POSTGRES_USER=${DB_USER}" \
    -e "POSTGRES_PASSWORD=${DB_PASSWORD}" \
    -e "POSTGRES_DB=${DB_NAME}" \
    docker.io/library/postgres:14.1 \
    postgres -N 1000
    # ^ Increased maximum number of connections for testing purposes
fi


# Keep pinging Postgres until it's ready to accept commands
echo -n "===> Waiting for PostgreSQL to start up..." >&2
export PGPASSWORD="${DB_PASSWORD}"
until psql -h localhost -U "$DB_USER" -p "$DB_PORT" -d postgres -c '\q' &>/dev/null; do
  echo -n . >&2
  sleep 1
done

echo ' done!'

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
echo "export DATABASE_URL='${DATABASE_URL}'"

echo "===> Creating database..."
sqlx database create
echo "===> Migrating database..."
sqlx migrate run
