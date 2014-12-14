# cli.rs

A toolkit for building command-line programs in Rust.

`cli` aims to

- reduce boilerplate
- canonicalize output formatting
- encourage adherence to the conventions of standard streams
  (e.g. when to print to stderr vs stdout)
- encourage appropriate use of exit statuses

Concretely, `cli` is a collection of simple functions for formatting
command-line output, defining flags, and (eventually) things like
integrating with sysexits.

It uses [getopts](http://doc.rust-lang.org/getopts/) for argument
parsing and thus currently inherits `getopt`'s `experimental`
stability level.
