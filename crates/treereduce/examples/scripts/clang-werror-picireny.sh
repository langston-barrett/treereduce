#!/bin/sh

clang -Werror -o /dev/null "${1:-bench.c}" > /dev/null 2>&1
