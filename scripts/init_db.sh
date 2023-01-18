#!/usr/bin/env bash

# https://www.gnu.org/software/bash/manual/html_node/The-Set-Builtin.html
set -x # print a trace of simple commands, and their arguments
set -eo pipefail # exit immediately if a pipeline returns a non-zero status

# check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
# check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME=${DB_NAME:=newsletter}
# check if a custom port has been set, otherwise default to '5432'
DB_PORT=${DB_PORT:=5432}

# launch postgres using Docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    # ^ increased maximum number of connections for testing purposes
