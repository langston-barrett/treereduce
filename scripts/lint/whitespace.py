#!/usr/bin/env python3

"""Find and fix whitespace problems"""

from argparse import ArgumentParser
from pathlib import Path
from sys import exit


def die(msg: str, /) -> None:
    print(msg)
    exit(1)


def error(msg: str, /, *, fix: bool = False) -> None:
    if fix:
        print(msg)
    else:
        die(msg)


def check_file(path: Path, /, *, fix: bool = False) -> None:
    if not path.is_file():
        return
    try:
        t = path.read_text()
    except UnicodeDecodeError:
        return
    fixed = []
    changed = False
    if not t.endswith("\n"):
        error(f"{path}: No trailing newline", fix=fix)
        changed = True
    if t.endswith("\n\n"):
        error(f"{path}: Multiple trailing newlines", fix=fix)
        changed = True
        t = t.rstrip()
    for no, line in enumerate(t.splitlines()):
        stripped = line.rstrip()
        fixed.append(stripped)
        if line != stripped:
            error(f"{path}:{no + 1}: Trailing whitespace", fix=fix)
            changed = True
    if changed and fix:
        path.write_text("\n".join(fixed) + "\n")


def check(paths: list[Path], /, *, fix: bool = False) -> None:
    for path in paths:
        if path.is_dir():
            files = list(path.rglob("*"))
        else:
            files = [path]
        for file in files:
            if not file.is_file():
                continue
            check_file(file, fix=fix)


parser = ArgumentParser(description=__doc__)
parser.add_argument("--fix", action="store_true")
parser.add_argument("paths", nargs="+", type=Path)
args = parser.parse_args()
check(args.paths, fix=args.fix)
