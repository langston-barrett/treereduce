// RUN: treereduce-rust -q -j 1 -o - -s %s -- match-c 'static A:' 2>&1 | FileCheck %s

// CHECK: static A: () = LongStructTypeName;
static A: LongStructTypeName = LongStructTypeName;
// CHECK-EMPTY:
