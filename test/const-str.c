// RUN: treereduce-c -q -j 1 -o - -s %s -- match-c 'STRING' 2>&1 | FileCheck %s

// CHECK: STRING = "";
const char *STRING = "big long string that should be replaced";
// CHECK-EMPTY:
