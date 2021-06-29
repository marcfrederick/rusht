# Rusht

Rusht is a simple Lisp written in Rust. It can be used through an included REPL. The crate consists of two sub-crates
implementing the interpreter and command line interface respectively.

Tokenizer, parser, and interpreter have been written manually. No parser combinator or parser generator like tools have
been used.

## Features

* Included CLI/REPL
    * Command history
    * Highlight matching braces
* Standard library
* Type coercion
  ```lisp
  (+ "100" 5)
  ```
* Variable definitions
  ```lisp
  (def x 5)
  ```
* Lambda expressions
  ```lisp
  (def add1 (func (a) (+ a 1)))
  ```
  
