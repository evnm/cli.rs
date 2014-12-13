//! A practical library for building command-line programs in Rust.
//!
//! `cli` aims to
//!
//! - reduce boilerplate
//! - canonicalize output formatting
//! - force programs to adhere to the conventions of standard streams (e.g. when
//!   to print to stderr vs stdout)
//!
//! `cli` uses [getopts](http://doc.rust-lang.org/getopts/) for command-line
//! argument parsing.

#![crate_name = "cli"]
#![crate_type="rlib"]

extern crate getopts;
use getopts::{OptGroup, short_usage};
use getopts::usage as getopts_usage;
use std::os;

mod test;

/// Construct a canonical usage string from a collection of `OptGroup`s.
///
/// Usage strings format:
///
///     Usage: <arv0> [option synopsis]...
///
///     Options:
///         [option description]...
pub fn usage(opts: &[OptGroup]) -> String {
    let args = os::args();
    let argv0 = args[0].clone();
    format!("{}", getopts_usage(short_usage(argv0.as_slice(), opts).as_slice(), opts))
}
