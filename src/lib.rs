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
//! Concretely, `cli` is a collection of simple functions for formatting
//! command-line output, defining flags, and (eventually) things like
//! integrating with sysexits.
//!
//! It uses [getopts](http://doc.rust-lang.org/getopts/) for argument parsing
//! and thus currently inherits `getopt`'s `experimental` stability level.
//!
//! ## Usage
//!
//! The following example shows how to integrate `cli` functions into a program's
//! `main` function in order to avoid `getopts` boilerplate. Sane defaults are
//! provided for help and version output without forcing consuming code to buy into
//! an application framework.
//!
//!     extern crate cli;
//!     extern crate getopts;
//!
//!     fn main() {
//!         let opts = &[
//!             cli::helpopt(),
//!             cli::versionopt(),
//!             getopts::optopt("o", "", "Set output file name", "FILENAME"),
//!         ];
//!
//!         let matches = cli::parse_args(opts);
//!         if matches.opt_present("h") {
//!             println!("{}", cli::usage_string(opts));
//!             return;
//!         }
//!         if matches.opt_present("version") {
//!             println!("{}", cli::version_string("0.0.1"));
//!             return;
//!         }
//!
//!         ...
//!     }

#![crate_name = "cli"]
#![experimental]
#![crate_type="rlib"]

extern crate getopts;
use getopts::{Matches, OptGroup, getopts, optflag, short_usage, usage};
use std::{io, os};
use std::io::fs;

mod test;

/// The file path of the executed program.
///
/// Typically the file path of the invoked executable. If the invocation target
/// is a symlink, then it will be resolved to the executable it links to.
pub fn exec_path() -> Path {
    let path = Path::new(os::args()[0].clone());
    fs::readlink(&path).unwrap_or(path)
}

/// Construct a canonical usage string from a collection of `OptGroup`s.
///
/// Usage string format:
///
///```ignore
///     Usage: <program name> [option synopsis]...
///
///     Options:
///         [option description]...
///```
pub fn usage_string(opts: &[OptGroup]) -> String {
    let exec_path = exec_path();
    let exec_path = exec_path.as_str().unwrap_or_else(|| "");
    format!("{}", usage(short_usage(exec_path, opts).as_slice(), opts))
}

/// Construct a version string.
///
/// Intended for use as output in response to `--version` as defined by
/// `cli::versionopt`.
///
/// Version string format:
///
///```ignore
///     <program name> version <version>
///```
pub fn version_string(version: &str) -> String {
    format!("{} version {}", exec_path().display(), version)
}

/// Parse the command-line arguments with which the program was executed
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
            match io::stderr().write_str(usage_string(opts).as_slice()) {
                Ok(()) => panic!(getopts_error.to_string()),
                Err(write_error) =>
                    // Write to stderr failed -- panic with both error messages.
                    panic!("{}\n{}", getopts_error.to_string(), write_error.to_string())
            }
        }
    }
}

/// Create a help flag `OptGroup`.
///
/// The returned `OptGroup` is an optional long option for the input `-h`
/// and `--help`.
pub fn helpopt() -> OptGroup {
    optflag("h", "help", "Print this help menu")
}

/// Create a version flag `OptGroup`.
///
/// The returned `OptGroup` is an optional long option for the input
/// `--version`. `-v` and `-V` are avoided in order to prevent confusion in the
/// event when a flag is needed for enabling verbose output.
pub fn versionopt() -> OptGroup {
    optflag(
        "",
        "version",
        format!("Print the version of {} being run", exec_path().display()).as_slice()
    )
}
