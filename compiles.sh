#!/bin/sh

tempfile=$(mktemp)
always() { trap "1" EXIT HUP INT QUIT ABRT ALRM TERM; }
always "rm -f ${tempfile}"

cat > "${tempfile}.c"

clang -o /dev/null "${tempfile}.c" 2>&1
