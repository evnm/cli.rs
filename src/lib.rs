//! A toolkit for building Unix command-line programs in Rust.
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
//! command-line output, defining flags, and adhering to Unix conventions.
//!
//! > **Warning**: Though unmarked, this library should be considered unstable, in
//! > part due to its usage of unstable language features.
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
//!     use getopts::Options;
//!
//!     fn main() {
//!         let mut opts = Options::new();
//!         cli::helpopt(&mut opts);
//!         cli::versionopt(&mut opts);
//!         opts.optopt("o", "", "Set output file name", "FILENAME");
//!
//!         let matches = cli::parse_args(&opts);
//!
//!         if matches.opt_present("h") {
//!             println!("{}", cli::usage_string(&opts));
//!             return;
//!         }
//!         if matches.opt_present("version") {
//!             println!("{}", cli::version_string("0.0.1"));
//!             return;
//!         }
//!
//!         // ...
//!     }
//!
//! When compiled to a binary named `foo`, this program emits the following output.
//!
//! ```ignore
//! $ foo -h
//! Usage: foo [-h] [--version] [-o FILENAME]
//!
//! Options:
//!     -h --help           Print this help menu
//!     --version           Print the version of target/cli being run
//!     -o FILENAME         Set output file name
//!
//! $ foo --version
//! foo version 0.0.1
//! ```

#![crate_name = "cli"]
#![crate_type="rlib"]

#![feature(collections)]
#![feature(io)]
#![feature(os)]
#![feature(path)]

extern crate getopts;
use getopts::{Matches, Options};
use std::{old_io, os};
use std::old_io::fs;

mod test;

/// A collection of predefined exit codes cribbed from
/// [sysexits.h](http://www.freebsd.org/cgi/man.cgi?query=sysexits).
///
/// From `sysexits.h`:
///
/// > According to style(9), it is not a good practice to call exit(3) with
/// > arbitrary values to indicate a failure condition when ending a
/// > program.  Instead, the pre-defined exit codes from sysexits should be
/// > used, so the caller of the process can get a rough estimation about
/// > the failure class without looking up the source code.
///
/// Intended for use with `std::os::set_exit_status` prior to process exit.
///
/// # Example
///
/// ```ignore
/// use std::os;
/// fn main() {
///   // ...
///   os::set_exit_status(cli::sysexits::USAGE);
///   return;
/// }
/// ```
pub mod sysexits {
    /// Successful termination
    pub const OK: isize = 0;
    /// Command line usage error
    pub const USAGE: isize = 64;
    /// Data format error
    pub const DATA_ERR: isize = 65;
    /// Cannot open input
    pub const NO_INPUT: isize = 66;
    /// Addressee unknown
    pub const NO_USER: isize = 67;
    /// Host name unknown
    pub const NO_HOST: isize = 68;
    /// Service unavailable
    pub const UNAVAILABLE: isize = 6;
    /// Internal software error
    pub const SOFTWARE_ERR: isize = 70;
    /// System error (e.g. can't fork)
    pub const OS_ERR: isize = 71;
    /// Critical OS file missing
    pub const  OS_FILE: isize = 72;
    /// Can't create (user) output file
    pub const CANT_CREAT: isize = 73;
    /// Input/output error
    pub const IO_ERR: isize = 74;
    /// Temp failure; user is invited to retry
    pub const TEMP_FAIL: isize = 75;
    /// Remote error in protocol
    pub const PROTOCOL: isize = 76;
    /// Permission denied
    pub const NO_PERM: isize = 77;
    /// Configuration error
    pub const CONFIG: isize = 78;
}

/// The file path of the executed program.
///
/// Typically the file path of the invoked executable. If the invocation target
/// is a symlink, then it will be resolved to the executable it links to.
pub fn exec_path() -> Path {
    let path = Path::new(os::args()[0].clone());
    fs::readlink(&path).unwrap_or(path)
}

/// Construct a canonical usage string from a collection of `Options`.
///
/// Usage string format:
///
/// ```ignore
/// Usage: <program name> [option synopsis]...
///
/// Options:
///     [option description]...
/// ```
pub fn usage_string(opts: &Options) -> String {
    let exec_path = exec_path();
    let exec_path = exec_path.as_str().unwrap_or_else(|| "");
    format!("{}", opts.usage(&opts.short_usage(exec_path)[]))
}

/// Construct a version string.
///
/// Intended for use as output in response to `--version` as defined by
/// `cli::versionopt`.
///
/// Version string format:
///
/// ```ignore
/// <program name> version <version>
/// ```
pub fn version_string(version: &str) -> String {
    format!("{} version {}", exec_path().display(), version)
}

/// Parse the command-line arguments with which the program was executed
/// according to a collection of `Options`.
///
/// Any flag parsing failure results in task panic. The program's usage string
/// is printed to stderr prior to panic. Panic is induced in order to avoid
/// program execution with undefined configuration. In such cases, the presence
/// of unrecognized flags or invalid flag values implies confusion on the part
/// of the executor. While perhaps overbearing, it is preferable to halt
/// execution abruptly than to continue with the risk of unwanted behavior.
pub fn parse_args(opts: &Options) -> Matches {
    match opts.parse(os::args().tail()) {
        Ok(matches) => matches,
        Err(getopts_error) => {
            // Write usage string to stderr, then panic.
            match old_io::stderr().write_str(&usage_string(opts)[]) {
                Ok(()) => panic!(getopts_error.to_string()),
                Err(write_error) =>
                    // Write to stderr failed -- panic with both error messages.
                    panic!("{}\n{}", getopts_error.to_string(), write_error.to_string())
            }
        }
    }
}

/// Add a help flag to `Options`.
///
/// The flag added is an optional long option for the input `-h`
/// and `--help`.
pub fn helpopt(opts: &mut Options) -> &mut Options {
    opts.optflag("h", "help", "Print this help menu")
}

/// Add a version flag to `Options`.
///
/// The flag added is an optional long option for the input
/// `--version`. `-v` and `-V` are avoided in order to prevent confusion in the
/// event when a flag is needed for enabling verbose output.
pub fn versionopt(opts: &mut Options) -> &mut Options {
    opts.optflag(
        "",
        "version",
        &format!("Print the version of {} being run", exec_path().display())[]
    )
}
