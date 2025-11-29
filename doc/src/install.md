# Installation

## From a Release

Statically-linked Linux binaries are available on the [releases page][releases].

## From crates.io

You can build a released version from [crates.io][crates-io]. You'll need the
Rust compiler and the [Cargo][cargo] build tool. [rustup][rustup] makes it very
easy to obtain these. Then, to install the reducer for the language `<LANG>`,
run:

```sh
cargo install treereduce-<LANG>
```

This will install binaries in `~/.cargo/bin` by default.

## From Source

You can also [build from source](build.md).

[#11]: https://github.com/langston-barrett/treereduce/issues/11
[cargo]: https://doc.rust-lang.org/cargo/
[crates-io]: https://crates.io/
[releases]: https://github.com/langston-barrett/treereduce/releases
[rustup]: https://rustup.rs/
