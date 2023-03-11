#!/bin/sh

# Interestingness test for C programs that compiles with Clang

tempfile=$(mktemp)
always() { trap "1" EXIT HUP INT QUIT ABRT ALRM TERM; }
always "rm -f ${tempfile}"

cat > "${tempfile}.c"

clang -o -Werror /dev/null "${tempfile}.c" > /dev/null 2>&1
