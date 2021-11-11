#!/bin/sh
set -e

gcc -lX11 cutewm.c
Xephyr -ac -screen 800x600 -br -reset 2> /dev/null :1 &
sleep 1
export DISPLAY=:1
./a.out &
xterm &
