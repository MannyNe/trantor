#!/usr/bin/bash

set -e

# run PG
docker compose up -d trantor_db

# wait for x seconds for the healthcheck to be finished, then populate the database
echo "waiting for container healthcheck to be finished..."
sleep 6

if [ $(docker ps -f health=healthy -f name=trantor_db -q | wc -l) -eq 1 ]; then
    cat ./migrations/20230523083741_init.sql | docker exec -i trantor_db psql -U trantor -d trantor
else
    echo "container isn't healthy or healthcheck hasn't started yet (timeout)"
    exit 1
fi

# check if user passes a tag for the image
if [ -z "$1"]; then
    VERSION="latest"
else
    VERSION=$1
fi

# build the image
docker build --network host -t trantor_backend:$VERSION .

# run docker compose
docker compose up -d trantor_backend