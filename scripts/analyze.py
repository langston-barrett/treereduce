#!/usr/bin/env python3

from collections import defaultdict
from json import loads as json_loads
from pathlib import Path


def fields_by_event_type(events, ty):
    return [e["fields"] for e in events if e["fields"]["event"] == ty]


def main():
    text = Path("log.jsonl").read_text()
    data = [json_loads(line) for line in text.splitlines()]
    events = [datum for datum in data if datum["fields"].get("event")]
    
    counts = defaultdict(lambda: 0)
    for e in events:
        f = e["fields"]
        if f["event"] == "push" and f["kind"] != "explore":
            counts[f["kind"]] += 1
    
    interesting = fields_by_event_type(events, "interesting")
    uninteresting = fields_by_event_type(events, "uninteresting")
    retries = fields_by_event_type(events, "retry")
    stales = fields_by_event_type(events, "stale")
    
    executions = []
    for e in data:
        if e.get("span", dict()).get("name", None) == "Waiting for command":
            executions += [e]

    # TODO(lb): Expected values of each reduction strategy (by node kind)
    print(f"Interesting deletions: {sum(1 for f in interesting if f['kind'] == 'delete')} / {counts['delete']}")
    print(f"Interesting mass deletions: {sum(1 for f in interesting if f['kind'] == 'delete_all')} / {counts['delete_all']}")
    print(f"Total interesting: {len(interesting)} / {sum(counts.values())}")
    print("Executions:", len(executions))
    print("Retries:", len(retries))
    print("Stales:", len(stales))

if __name__ == "__main__":
    main()
