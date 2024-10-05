#!/bin/sh

set -xe

clang -Wall -Wextra -ggdb -o ./bin/day03 -Idev-deps day03/main.c -lm
