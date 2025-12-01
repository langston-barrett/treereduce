#!/usr/bin/env python3

"""Check cross-references"""

from argparse import ArgumentParser
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from sys import exit, stderr


def die(msg: str) -> None:
    print(msg, file=stderr)
    exit(1)


@dataclass(frozen=True)
class Pos:
    file: Path
    line: int

    def __str__(self) -> str:
        return f"{self.file}:{self.line}"


def find_def_or_ref(s: str, pattern: str) -> str | None:
    i = s.find(pattern)
    if i == -1:
        return None
    s = s[i + len(pattern) :]
    j = s.find(" ")
    if j == -1:
        return s
    return s[:j]


def find_def(s: str) -> str | None:
    return find_def_or_ref(s, " def: ")


def find_ref(s: str) -> str | None:
    return find_def_or_ref(s, " ref: ")


def collect(
    path: Path,
    content: str,
    defs: dict[str, set[Pos]],
    refs: dict[str, set[Pos]],
) -> None:
    if path.name == "xref.py":
        return
    for n, line in enumerate(content.splitlines(), start=1):
        pos = Pos(path, n)
        if def_name := find_def(line):
            defs[def_name].add(pos)
        elif ref_name := find_ref(line):
            refs[ref_name].add(pos)


def check(defs: dict[str, set[Pos]], refs: dict[str, set[Pos]]) -> bool:
    ok = True
    # Check that every ref has a def
    for ref_name, positions in refs.items():
        if ref_name not in defs:
            ok = False
            for pos in positions:
                print(f"{pos}: Missing def for ref `{ref_name}`", file=stderr)
    # Check that every def has a ref
    for def_name, positions in defs.items():
        if def_name not in refs:
            ok = False
            for pos in positions:
                print(f"{pos}: Missing ref for def `{def_name}`", file=stderr)
    # Check that every def appears exactly once
    for def_name, positions in defs.items():
        n_defs = len(positions)
        if n_defs != 1:
            ok = False
            for i, pos in enumerate(positions):
                print(f"{pos}: def {i} of {n_defs} for `{def_name}`", file=stderr)
    return ok


def go(paths: list[Path]) -> None:
    defs: dict[str, set[Pos]] = defaultdict(set)
    refs: dict[str, set[Pos]] = defaultdict(set)
    paths = list(set(paths))

    while paths:
        path = paths.pop()
        if path.is_dir():
            paths.extend(path.iterdir())
        else:
            try:
                content = path.read_text()
                collect(path, content, defs, refs)
            except UnicodeDecodeError:
                pass
            except Exception as e:
                die(f"Error reading {path}: {e}")

    if not check(defs, refs):
        exit(1)


parser = ArgumentParser(description=__doc__)
parser.add_argument("paths", nargs="+", type=Path)
args = parser.parse_args()
go(args.paths)
