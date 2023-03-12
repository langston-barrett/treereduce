// RUN: true
// COM: XFAIL
// COM: treereduce-c -q -j 1 -o - -s %s -- clang -c -o /dev/null @@.c 2>&1 | FileCheck %s

// CHECK: STRING = "";
const char *STRING = "big long string that should be replaced";
// CHECK-EMPTY:
