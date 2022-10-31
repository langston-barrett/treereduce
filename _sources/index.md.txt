# treedd

`treedd` is a fast, parallel, syntax-aware, multi-language program reducer based
on delta debugging and tree-sitter grammars. In other words, `treedd` helps you
shrink programs while maintaining some property of interest, for example, that
the program causes a compiler crash or outputs a certain message.

Source available [on Github][src].

`treedd` currently supports the following languages:

- C

```{toctree}
:caption: User Guide

changelog
comparison
contributing
install
usage
```

```{toctree}
:caption: Reference

algorithm
```

```{toctree}
:caption: Developer Guide

build
dev
```

[src]: https://github.com/langston-barrett/treeddb
