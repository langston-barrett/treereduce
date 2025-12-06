# Linting and formatting

We employ a variety of linting and formatting tools. They can be run manually or
with [Ninja].

[Ninja]: https://ninja-build.org/

## Ninja script

To run all the linters:

```sh
./scripts/lint/lint.py
```

To run all the formatters:

```sh
./scripts/lint/lint.py --format
```

As a [pre-commit hook]:

[pre-commit hook]: https://git-scm.com/docs/githooks#_pre_commit

```
cat <<'EOF' > .git/hooks/pre-commit
#!/usr/bin/env bash
./scripts/lint/lint.py
EOF
chmod +x .git/hooks/pre-commit
```

## Clippy

We lint Rust code with [Clippy][clippy].

[clippy]: https://doc.rust-lang.org/stable/clippy/

You can install Clippy with [`rustup`] like so:

[`rustup`]: https://rustup.rs/

```sh
rustup component add clippy
```

and run it like this:

```sh
cargo clippy --all-targets -- --deny warnings
```

## Generic scripts

We have a few Python scripts in `scripts/lint/` that perform one-off checks.
They generally take some number of paths as arguments. Use their `--help`
options to learn more.

## mdlynx

We run [mdlynx] on our Markdown files to check for broken links.

[mdlynx]: https://github.com/langston-barrett/mdlynx

```bash
git ls-files -z --exclude-standard '*.md' | xargs -0 mdlynx
```

## Mypy

We lint Python code with [mypy] in `--strict` mode.

[mypy]: https://www.mypy-lang.org/

```sh
git ls-files -z --exclude-standard '*.py' | xargs -0 mypy --strict
```

## Ruff

We lint and format Python code with [Ruff].

[Ruff]: https://docs.astral.sh/ruff/

```sh
git ls-files -z --exclude-standard '*.py' | xargs -0 ruff format
git ls-files -z --exclude-standard '*.py' | xargs -0 ruff check
```

## `rustfmt`

We format Rust code with [`rustfmt`].

[rustfmt]: https://rust-lang.github.io/rustfmt

You can install rustfmt with [`rustup`] like so:

[rustup]: https://rustup.rs/

```sh
rustup component add rustfmt
```

and then run it like this:

```sh
cargo fmt --all
```

## ShellCheck

We lint shell scripts with [ShellCheck].

[ShellCheck]: https://www.shellcheck.net/

```sh
git ls-files -z --exclude-standard '*.sh' | xargs -0 shellcheck
```

## taplo

We format TOML files with [taplo].

[taplo]: https://taplo.tamasfe.dev/

```bash
git ls-files -z --exclude-standard '*.toml' | xargs -0 taplo format
```

## ttlint

We lint text files with [ttlint].

[ttlint]: https://github.com/langston-barrett/ttlint

```bash
git ls-files -z --exclude-standard '**' | xargs -0 ttlint
```

## typos

We run [typos] on Markdown files.

[typos]: https://github.com/crate-ci/typos

```bash
git ls-files -z --exclude-standard '*.md' | xargs -0 typos
```

## zizmor

We lint our GitHub Actions files with [zizmor].

[zizmor]: https://docs.zizmor.sh/

```bash
zizmor .github
```
