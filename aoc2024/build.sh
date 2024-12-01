#!/bin/sh

set -xe

clang -Wall -Wextra -ggdb -o ./bin/day01 -Idev-deps day01/main.c -lm
