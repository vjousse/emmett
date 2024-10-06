#!/usr/bin/env bash

trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
simple-http-server -i output &
watchexec -e md,html,rs,css cargo run -- -p
