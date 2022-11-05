# treereduce

`treereduce` is a fast, parallel, syntax-aware test case reducer based on
tree-sitter grammars. In other words, `treereduce` helps you shrink structured
data (especially source code) while maintaining some property of interest, for
example, that the program causes a compiler crash or outputs a certain message.
See [the documentation](./doc) for more information.
