#!/usr/bin/env zsh
#set -x
set -eo pipefail

function startDocker () {
  while (! docker stats --no-stream &> /dev/null); do
    printf "."
    sleep 1
    done
    printf "\n"
}


function checkDocker() {
  if ! pgrep -x "docker" > /dev/null
  then
    printf "Starting docker"
    open /Applications/Docker.app
    startDocker
    fi
}

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  echo >&2"to install it."
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=rustwebdev}"
DB_PORT="${POSTGRES_PORT:=5432}"

if [[ -z "${SKIP_DOCKER}" ]]
then
checkDocker

docker run --name rustwebdev \
  -e DB_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres:14.1-alpine3.15 \
  postgres -N 1000
fi

# Ping Postgres until it's ready to accept commands.
export PGPASSWORD=${DB_PASSWORD}
printf "Postgres is unavailable - sleeping"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' &> /dev/null ; do
  printf "."
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"