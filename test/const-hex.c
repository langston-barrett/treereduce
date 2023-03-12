// RUN: treereduce-c -q -j 1 -o - -s %s -- grep -E 'long NUM =' 2>&1 | FileCheck %s

// CHECK: NUM = 0;
const long NUM = 0xbad1dea;
// CHECK-EMPTY:
