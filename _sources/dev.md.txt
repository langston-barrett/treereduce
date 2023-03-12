# Development

To get set up to build from source, see {doc}`build`.

## Tools

In addition to Cargo and rustc, you'll need `clippy` to lint your code.

## Testing

Tested with `lit` and `FileCheck `. 

```sh
cargo build
lit --path=$PWD/test/bin --path=$PWD/target/debug test/
```

## Tuning

### Benchmarking

(profiling)=
### Profiling

Profiling multi-threaded programs is hard. Use the included [Poor Man's
Profiler][poor-man] like so:

Start the task you want to profile:

```sh
cargo run --bin treereduce-c -- -j 12 --output - -s ./crates/treereduce/benches/c/hello-world-big.c 'clang -o /dev/null @@.c'
```

In a separate terminal:

```sh
./scripts/profile.sh |& tee prof.log
```

## Releasing

- Create branch with a name starting with `release`
- Update `doc/changelog.md`
- Update the version number in `Cargo.toml`, then `cargo build --release`
- Check that CI was successful on the release branch
- Merge the release branch to `main`
- Delete the release branch
- `git checkout main && git pull origin && git tag -a vX.Y.Z -m vX.Y.Z && git push --tags`
- Verify that the release artifacts work as intended
- Release the pre-release created by CI
- Check that the crates were properly uploaded to crates.io

[poor-man]: http://poormansprofiler.org/
