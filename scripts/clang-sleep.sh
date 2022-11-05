#!/bin/sh

# Interestingness test for C programs that compiles with Clang

if [[ -n "${CREDUCE}" ]]; then
  sleep 0.5
  clang -o /dev/null "${CREDUCE}" > /dev/null 2>&1
else
  tempfile=$(mktemp)
  always() { trap "1" EXIT HUP INT QUIT ABRT ALRM TERM; }
  always "rm -f ${tempfile}"

  cat > "${tempfile}.c"

  sleep 0.5
  clang -o /dev/null "${tempfile}.c" > /dev/null 2>&1
fi
