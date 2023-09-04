#!/bin/bash

PRIVATE_KEY="lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua" \
PG_DB_NAME=sankar \
PG_DB_UNAME=sankar \
PG_DB_PWD=sankar \
REDIS_URI="127.0.0.1:6379" \
DB_URI="127.0.0.1:9042" \
LP_HOST=localhost \
DEV=TRUE \
LP_PORT=9901 \
./target/release/neo4j

# sudo lsof -i -P -n | grep LISTEN 