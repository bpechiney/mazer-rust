//! The entrpoint of mazer

#![warn(missing_docs)]

mod cli;
mod logging;
mod search;

use clap::Parser;

use crate::cli::Opts;
use crate::logging::init_logging;

fn main() {
    let opts = Opts::parse();

    init_logging(opts.log_level);

    println!("Hello, world!");
}
