// RUN: true
// COM: XFAIL
// COM: treereduce-c -q -j 1 -o - -s %s -- clang -c -o /dev/null @@.c 2>&1 | FileCheck %s

// CHECK: NUM = 0x0;
const long NUM = 0xbad1dea;
// CHECK-EMPTY:
