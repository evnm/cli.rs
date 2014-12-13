# cli.rs

A practical library for building command-line programs in Rust.

`cli` aims to

- reduce boilerplate
- canonicalize output formatting
- force programs to adhere to the conventions of standard streams (e.g. when
  to print to stderr vs stdout)

`cli` uses [getopts](http://doc.rust-lang.org/getopts/) for command-line
argument parsing.
