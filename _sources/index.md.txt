# treereduce

`treereduce` is a fast, parallel, syntax-aware test case reducer based on
tree-sitter grammars. In other words, `treereduce` helps you shrink structured
data (especially source code) while maintaining some property of interest, for
example, that the program causes a compiler crash or outputs a certain message.

See the {doc}`overview` for more information, or get started right away with
{doc}`install` and {doc}`usage`.

Source available [on Github][src].

```{toctree}
:caption: User Guide

contributing
install
usage
```

```{toctree}
:caption: Reference

benchmarks
changelog
design
overview
```

```{toctree}
:caption: Developer Guide

build
dev
```

[src]: https://github.com/langston-barrett/treereduceb
