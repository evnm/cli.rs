# cli.rs

A toolkit for building Unix command-line programs in Rust.

`cli` aims to

- reduce boilerplate
- canonicalize output formatting
- encourage adherence to the conventions of standard streams
  (e.g. when to print to stderr vs stdout)
- encourage appropriate use of exit statuses

Concretely, `cli` is a collection of simple functions for formatting
command-line output, defining flags, and adhering to Unix conventions.

> **Warning**: Though unmarked, this library should be considered unstable, in
> part due to its usage of unstable language features.

## Usage

The following example shows how to integrate `cli` functions into a program's
`main` function in order to avoid `getopts` boilerplate. Sane defaults are
provided for help and version output without forcing consuming code to buy into
an application framework.

    extern crate cli;
    extern crate getopts;

    use getopts::Options;

    fn main() {
        let mut opts = Options::new();
        cli::helpopt(&mut opts);
        cli::versionopt(&mut opts);
        opts.optopt("o", "", "Set output file name", "FILENAME");

        let matches = cli::parse_args(&opts);

        if matches.opt_present("h") {
            println!("{}", cli::usage_string(&opts));
            return;
        }
        if matches.opt_present("version") {
            println!("{}", cli::version_string("0.0.1"));
            return;
        }

        ...
    }

When compiled to a binary named `foo`, this program emits the following output.

    $ foo -h
    Usage: foo [-h] [--version] [-o FILENAME]

    Options:
        -h --help           Print this help menu
        --version           Print the version of target/cli being run
        -o FILENAME         Set output file name
    $ foo --version
    foo version 0.0.1

## Documentation

Generated `rustdoc` website is available at
http://evanmeagher.net/cli.rs/doc/cli/.
