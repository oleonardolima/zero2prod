#!/usr/bin/env bash

# https://www.gnu.org/software/bash/manual/html_node/The-Set-Builtin.html
set -x # print a trace of simple commands, and their arguments
set -eo pipefail # exit immediately if a pipeline returns a non-zero status

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres,rustls"
    echo >&2 "to install it."
    exit 1
fi

# check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
# check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME=${DB_NAME:=newsletter}
# check if a custom port has been set, otherwise default to '5432'
DB_PORT=${DB_PORT:=5432}

# allow to skipt Docker postgres launch if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
        # ^ increased maximum number of connections for testing purposes
fi

# keep checking Postgres until it's ready to accept connections/commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still not available - retrying"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# it expects the `DATABASE_URL` env var, as a valid Postgres connection
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

# please check the `sqlx-cli database create --help` for more info
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"