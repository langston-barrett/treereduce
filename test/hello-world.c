// RUN: treereduce-c -j 1 -o - -s %s -- clang -o /dev/null @@.c 2>&1 | FileCheck %s

#include <stdio.h>
// CHECK: main()
int main(int argc, char *argv[]) {
// CHECK-NOT: puts
  puts("Hello, world!\n");
  return 0;
}
// CHECK-EMPTY:
