#!/bin/sh

# http://poormansprofiler.org/

set -eu

nsamples=20
sleeptime=0.1
pid=$(pgrep "${1:-treereduce}")

for x in $(seq 1 "${nsamples}"); do
  gdb -q -ex "set pagination 0" -ex "thread apply all bt" -batch -p ${pid}
  sleep "${sleeptime}"
done | \
awk '
  BEGIN { s = ""; } 
  /^Thread/ { print s; s = ""; } 
  /^\#/ { if (s != "" ) { s = s "," $4} else { s = $4 } } 
  END { print s }' | \
sort | uniq -c | sort -r -n -k 1,1
