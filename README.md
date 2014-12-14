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

## Usage

The following example shows how to integrate `cli` functions into a program's
`main` function in order to avoid `getopts` boilerplate. Sane defaults are
provided for help and version output without forcing consuming code to buy into
an application framework.

    extern crate cli;
    extern crate getopts;

    fn main() {
        let opts = &[
            cli::helpopt(),
            cli::versionopt(),
            getopts::optopt("o", "", "Set output file name", "FILENAME"),
        ];

        let matches = cli::parse_args(opts);
        if matches.opt_present("h") {
            println!("{}", cli::usage_string(opts));
            return;
        }
        if matches.opt_present("version") {
            println!("{}", cli::version_string("0.0.1"));
            return;
        }

        ...
    }

## Documentation

Generated `rustdoc` website is available at
http://evanmeagher.net/cli.rs/doc/cli/.
