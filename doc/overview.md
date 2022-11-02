# Overview

## Features

- **Fast**: `treedd` uses a {doc}`novel algorithm <design>` for parallelized
  reduction of tree-shaped data, based on ideas from {ref}`recent research
  <bib>`. It is written in Rust and has been {doc}`extensively benchmarked
  <benchmarks>`.
- **Effective**: `treedd` produces {doc}`small programs <benchmarks>`.
- **Robust**: `treedd` is based on tree-sitter grammars, which are robust to
  parse errors. This means you can reduce syntactically invalid inputs, and
  each grammar doesn't need to be 100% perfect to work for all programs.
- **Easy to set up**: `treedd` reducers are distributed as static binaries.
- **Multi-language**: `treedd` currently supports the following languages:

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

[halfempty]: https://github.com/googleprojectzero/halfempty
[comby-reducer]: https://github.com/comby-tools/comby-reducer
[perses]: https://github.com/uw-pluverse/perses
[picireny]: https://github.com/renatahodovan/picireny
[gtr]: https://github.com/sherfert/GTR
