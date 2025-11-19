use clap::Parser;

use crate::logging::LogLevel;
use crate::search::algorithms::SearchAlgorithm;

/// Top-level command line options.
#[derive(Debug, Parser)]
#[command(name = "mazer", version, about = "A maze-solving CLI tool.", long_about = None)]
pub struct Opts {
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = LogLevel::Info,
        help = "The level at which logging should occur."
    )]
    pub log_level: LogLevel,

    #[arg(
        short,
        long,
        value_enum,
        help = "The algorithm with which to solve the maze."
    )]
    pub algorithm: SearchAlgorithm,
}
