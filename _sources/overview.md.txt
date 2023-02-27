# Overview

## Features

- **Fast**: `treereduce` uses a {doc}`novel algorithm <design>` for parallelized
  reduction of tree-shaped data, based on ideas from {ref}`recent research
  <bib>`. It has been {doc}`benchmarked <benchmarks>` against similar tools.
- **Effective**: `treereduce` produces {doc}`small programs <benchmarks>`.
- **Robust**: `treereduce` is based on tree-sitter grammars, which are robust to
  parse errors. This means you can reduce syntactically invalid inputs, and
  each grammar doesn't need to be 100% perfect to work for all programs.
- **Easy to set up**: `treereduce` reducers are distributed as static binaries.
- **Multi-language**: `treereduce` currently supports the following languages:

  * C
  * C++

```{warning}
TODO([#13][#13])
```

## Comparison to Other Tools

Test-case reduction tools form a spectrum: tools that are completely agnostic
to the input format (e.g., Halfempty) are applicable in more situations, but
will likely perform worse than highly-specialized tools (e.g., C-reduce).
`treereduce` is somewhere in the middle: it is aware of the *syntax* of inputs,
and works on a variety of different languages.

Perses and Picireny are also syntax-aware; they use ANTLR rather than tree-
sitter grammars (making them unable to mutate malformed inputs). The goal of
`treereduce` is to be faster and/or easier to use than these tools.

The following table lists several test-case reduction tools:

| Tool                             | Input    | Grammar     | Parallel |
|----------------------------------|----------|-------------|----------|
| [comby-reducer][comby-reducer]   | C-like   | n/a         |          |
| [C-Reduce][creduce]              | C        | n/a         | ✅       |
| [GTR][gtr]                       | not sure | not sure    | ?        |
| [Halfempty][halfempty]           | any      | n/a         | ✅       |
| [Perses][perses]                 | \[note\] | ANTLR       | ?        |
| [Picireny][picireny]             | any      | ANTLR       | ✅       |
| `treereduce`                     | any      | tree-sitter | ✅       |

\[note\]: Perses supports the following languages:

- C
- Rust
- Java 8
- Go
- System Verilog

## FAQ

### How can I get results faster?

Try `--fast`. If that's not fast enough, read on.

The following tips are in order of descending utility, the last few will
technically make things a bit faster but the gains will be very minor.

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

Try `--slow`. If that's not small enough, read on.

- Use `--stable`. If that's too slow, increase `--passes`.
- Set `--min-reduction 1`.
- Run [Halfempty][halfempty] or another test-case reducer on the output.

[creduce]: https://embed.cs.utah.edu/creduce/
[halfempty]: https://github.com/googleprojectzero/halfempty
[comby-reducer]: https://github.com/comby-tools/comby-reducer
[perses]: https://github.com/uw-pluverse/perses
[picireny]: https://github.com/renatahodovan/picireny
[gtr]: https://github.com/sherfert/GTR
