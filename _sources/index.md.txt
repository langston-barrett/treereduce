# treedd

`treedd` is a fast, parallel, syntax-aware, multi-language program reducer based
on delta debugging and tree-sitter grammars. In other words, `treedd` helps you
shrink programs while maintaining some property of interest, for example, that
the program causes a compiler crash or outputs a certain message.

See the {doc}`overview` for more information, or get started right away with
{doc}`install` and {doc}`usage`.

Source available [on Github][src].

```{warning}
`treedd` is **alpha-quality** software. Some parts of the documentation are
currently aspirational---these are marked with TODOs and linked issues.
```

```{toctree}
:caption: User Guide

changelog
contributing
install
overview
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
