# cli.rs

A toolkit for building command-line programs in Rust.

`cli` aims to

- reduce boilerplate
- canonicalize output formatting
- encourage adherence to the conventions of standard streams
  (e.g. when to print to stderr vs stdout)
- encourage appropriate use of exit statuses

`cli` uses [getopts](http://doc.rust-lang.org/getopts/) for
command-line argument parsing and thus currently inherits its
`experimental` stability level.
