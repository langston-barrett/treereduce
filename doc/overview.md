# Overview

## Features

- **Fast**: `treereduce` uses a {doc}`novel algorithm <design>` for parallelized
  reduction of tree-shaped data, based on ideas from {ref}`recent research
  <bib>`. It is written in Rust and has been {doc}`extensively benchmarked
  <benchmarks>`.
- **Effective**: `treereduce` produces {doc}`small programs <benchmarks>`.
- **Robust**: `treereduce` is based on tree-sitter grammars, which are robust to
  parse errors. This means you can reduce syntactically invalid inputs, and
  each grammar doesn't need to be 100% perfect to work for all programs.
- **Easy to set up**: `treereduce` reducers are distributed as static binaries.
- **Multi-language**: `treereduce` currently supports the following languages:

  * C

```{warning}
TODO([#13][#13])
```

## Comparison to Other Tools

```{warning}
TODO([#12][#12])
```

- [Halfempty][halfempty]
- [comby-reducer][comby-reducer]
- [Perses][perses]
- [Picireny][picireny]
- [GTR][gtr]

## FAQ

### How can I get results faster?

These tips are in order of descending utility, the last few will technically
make things a bit faster but the gains will be very minor.

- Try `--passes 1`.
- Set `--jobs` to something close to your number of CPU cores.
<!-- TODO(#6): --interesting-stdout-regex -->
- Pass the input to your program on stdin instead of via a file. If your program
  must take a file, put it on a tmpfs.
- Avoid using a script to wrap your interestingness test if you can, using
  `--interesting-exit-code` instead.
- For really slow tests, use `--no-verify` once you've set up your
  interestingness test.
- If the input file is generated, pass it to `treereduce` on stdin.
- If you don't need the output to be a file, pass `--output -` to get output on
  stdout.

### How can I get smaller tests?

Try increasing `--passes`, using `--stable`, and running [Halfempty][halfempty]
on the output.

[halfempty]: https://github.com/googleprojectzero/halfempty
[comby-reducer]: https://github.com/comby-tools/comby-reducer
[perses]: https://github.com/uw-pluverse/perses
[picireny]: https://github.com/renatahodovan/picireny
[gtr]: https://github.com/sherfert/GTR
