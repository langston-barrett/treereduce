# treedd

`treedd` is a fast, parallel, syntax-aware, multi-language program reducer based
on recursive hierarchical delta debugging with hoisting and tree-sitter
grammars. In other words, `treedd` helps you shrink programs while maintaining
some property of interest, for example, that the program causes a compiler crash
or outputs a certain message. See [the documentation](./doc) for more
information.
