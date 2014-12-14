//! A toolkit for building command-line programs in Rust.
//!
//! `cli` aims to
//!
//! - reduce boilerplate
//! - canonicalize output formatting
//! - encourage adherence to the conventions of standard streams
//!   (e.g. when to print to stderr vs stdout)
//! - encourage appropriate use of exit statuses
//!
//! `cli` uses [getopts](http://doc.rust-lang.org/getopts/) for
//! command-line argument parsing and thus currently inherits its
//! `experimental` stability level.

#![crate_name = "cli"]
#![crate_type="rlib"]

extern crate getopts;
use getopts::{Matches, OptGroup, getopts, short_usage};
use getopts::usage as getopts_usage;
use std::{io, os};

mod test;

/// Constructs a canonical usage string from a collection of `OptGroup`s.
///
/// Usage strings format:
///
///```ignore
///     Usage: <argv0> [option synopsis]...
///
///     Options:
///         [option description]...
///```
pub fn usage(opts: &[OptGroup]) -> String {
    let argv0 = os::args()[0].clone();
    format!("{}", getopts_usage(short_usage(argv0.as_slice(), opts).as_slice(), opts))
}

/// Parses the command-line arguments with which the program was executed
/// according to a collection of `OptGroup`s.
///
/// Any flag parsing failure results in task panic. The program's usage string
/// is printed to stderr prior to panic. Panic is induced in order to avoid
/// program execution with undefined configuration. In such cases, the presence
/// of unrecognized flags or invalid flag values implies confusion on the part
/// of the executor. While perhaps overbearing, it is preferable to halt
/// execution abruptly than to continue with the risk of unwanted behavior.
pub fn parse_args(opts: &[OptGroup]) -> Matches {
    match getopts(os::args().tail(), opts) {
        Ok(matches) => matches,
        Err(getopts_error) => {
            // Write usage string to stderr, then panic.
            match io::stderr().write_str(usage(opts).as_slice()) {
                Ok(()) => panic!(getopts_error.to_string()),
                Err(write_error) =>
                    // Write to stderr failed -- panic with both error messages.
                    panic!("{}\n{}", getopts_error.to_string(), write_error.to_string())
            }
        }
    }
}
