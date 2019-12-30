# Parsers in Rust

This is just playing around with some simple parsing techniques. I try to
make them as simple as possible. This is probably not a "best practice" when
it comes to implementing proper parsers.

I'm just parsing simple math expressions. The goal is to have input like this:

```
1 + 2 + 4
2 * 100
```

Turned in to a simple AST where each line is a `parent` with the numbers and
operators as child nodes.

1. Recursive descent parser

I've implemented these just loosely following any patterns. First the input
is lexed into a token stream. Then this stream is parsed. There is currently no
need to do any backtracing or anything fancy.

Loosely inspired by this: https://adriann.github.io/rust_parser.html

2. Recursive ascent parser

This is planning on using the recursive ascent parser based on this article:
https://www.abubalay.com/blog/2018/04/08/recursive-ascent


3. Using parser generators

This excellent article outlines how to use parser generators:
https://bodil.lol/parser-combinators/
https://matklad.github.io/2018/06/06/modern-parser-generator.html


## Status

Work in progress. Maybe some day I'll write about this, I don't know yet. 
This is currenty me just playing around.