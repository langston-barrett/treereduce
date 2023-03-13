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

  - C
  - Java
  - JavaScript
  - Rust
  - [Soufflé][souffle]

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
| `treereduce`                     | \[note\] | tree-sitter | ✅       |

\[note\]: Perses supports the following languages:

- C
- Rust
- Java 8
- Go
- System Verilog

`treereduce` currently supports the languages listed above.

[comby-reducer]: https://github.com/comby-tools/comby-reducer
[creduce]: https://embed.cs.utah.edu/creduce/
[gtr]: https://github.com/sherfert/GTR
[halfempty]: https://github.com/googleprojectzero/halfempty
[perses]: https://github.com/uw-pluverse/perses
[picireny]: https://github.com/renatahodovan/picireny
[souffle]: https://souffle-lang.github.io/index.html
