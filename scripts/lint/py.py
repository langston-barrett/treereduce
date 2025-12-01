#!/usr/bin/env python3

"""Lint Python scripts"""

from argparse import ArgumentParser
from pathlib import Path
from sys import exit


def die(msg: str) -> None:
    print(msg)
    exit(1)


def check(paths: list[Path]) -> None:
    for path in paths:
        if path.is_dir():
            files = list(path.rglob("*"))
        else:
            files = [path]
        for file in files:
            if not file.is_file():
                continue
            t = file.read_text()
            if '"""' not in t:
                die(f"{file}: Expected docstring")
            if "ArgumentParser(description=__doc__" not in t:
                die(f"{file}: Expected `ArgumentParser` with `description=__doc__`")


parser = ArgumentParser(description=__doc__)
parser.add_argument("paths", nargs="+", type=Path)
args = parser.parse_args()
check(args.paths)
