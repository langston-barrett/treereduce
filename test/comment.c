// RUN: treereduce-c -q -j 1 -o - -s %s -- grep "retain" 2>&1 | FileCheck %s

// CHECK: retain
// This comment should be retained.
// CHECK-NOT: discarded
// This comment should be discarded.
