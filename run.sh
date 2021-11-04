#!/bin/sh
set -e

cargo build
Xephyr -ac -screen 800x600 -br -reset -terminate 2> /dev/null :1 &
sleep 1
xterm -display :1 ./target/debug/cutewm
