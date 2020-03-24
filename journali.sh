#!/usr/bin/env bash
case "$1" in
start)
    docker-compose up -d
    yarn watch:dev
    exit 0
    ;;
stop)
    docker-compose down
    exit 0
    ;;
restart)
    docker-compose restart api
    exit 0
    ;;
esac
