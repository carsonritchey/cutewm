#!/bin/sh
set -e

cargo build
Xephyr -ac -screen 800x600 -br -reset 2> /dev/null :1 &
sleep 1
export DISPLAY=:1
./target/debug/cutewm &
xterm &
