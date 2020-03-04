#!/usr/bin/env bash
echo "Starting migration"
docker run --rm \
  -v "$(pwd)":/app \
  --network="test-rocket-app_default" \
  willsquire/diesel-cli \
  migration run
echo "Finished migrating"
