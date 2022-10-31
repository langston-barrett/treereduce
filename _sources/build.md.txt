# Build

To build from source, you'll need the Rust compiler and the [Cargo][cargo] build
tool. [rustup][rustup] makes it very easy to obtain these. Then, get the source:

```bash
git clone https://github.com/langston-barrett/treedd
cd treedd
```

Finally, build everything:

```bash
cargo build --release
```

You can find binaries in `target/release`. Run tests with `cargo test`.

[cargo]: https://doc.rust-lang.org/cargo/
[crates-io]: https://crates.io/
[rustup]: https://rustup.rs/
