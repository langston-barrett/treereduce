#!/usr/bin/env python3

"""Analyze reductions"""

from argparse import ArgumentParser
from collections import defaultdict
from dataclasses import dataclass
from enum import Enum, unique
from json import loads as json_loads
from pathlib import Path


@unique
class ReductionTy(str, Enum):
    DELETE = "delete"
    DELETE_ALL = "delete_all"


@dataclass(frozen=True, order=True)
class Reduction:
    ty: ReductionTy
    interesting: bool
    priority: int


def fields_by_event_type(events, ty):
    return [e["fields"] for e in events if e["fields"]["event"] == ty]


def reductions(events):
    for e in events:
        f = e["fields"]
        if not f["event"].endswith("interesting"):
            continue

        interesting = False
        if f["event"] == "interesting":
            interesting = True

        yield Reduction(ReductionTy(f["kind"]), interesting, f["priority"])


def expected_value(reds, ty):
    bytes_removed = sum(r.priority for r in reds if ty is None or r.ty == ty)
    successes = sum(1 for r in reds if r.interesting and (ty is None or r.ty == ty))
    tries = sum(1 for r in reds if ty is None or r.ty == ty)
    if tries == 0:
        return 0
    return bytes_removed * (successes / float(tries))


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
    # uninteresting = fields_by_event_type(events, "uninteresting")
    retries = fields_by_event_type(events, "retry")
    stales = fields_by_event_type(events, "stale")

    executions = []
    for e in data:
        if e.get("span", dict()).get("name", None) == "Waiting for command":
            executions += [e]

    reds = list(reductions(events))

    print(
        f"Interesting deletions: {sum(1 for f in interesting if f['kind'] == 'delete')} / {counts['delete']}"
    )
    print(
        f"Interesting mass deletions: {sum(1 for f in interesting if f['kind'] == 'delete_all')} / {counts['delete_all']}"
    )
    print(f"Total interesting: {len(interesting)} / {sum(counts.values())}")
    print("Executions:", len(executions))
    print("Retries:", len(retries))
    print("Stales:", len(stales))
    print("E[delete]:", expected_value(reds, ReductionTy.DELETE))
    print("E[delete_all]:", expected_value(reds, ReductionTy.DELETE_ALL))


if __name__ == "__main__":
    parser = ArgumentParser(description=__doc__)
    parser.parse()
    main()
