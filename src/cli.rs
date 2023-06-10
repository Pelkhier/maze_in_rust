use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Cli {
    #[arg(short, long)]
    /// The number of rows of the maze
    pub rows: usize,
    #[arg(short, long)]
    /// The number of columns of the maze
    pub columns: usize,
    #[arg(long = "sr")]
    /// Where the starting point should be in rows
    pub start_row: usize,
    #[arg(long = "sc")]
    /// Where the starting point should be in columns
    pub start_column: usize,
    #[arg(long = "tr")]
    /// Where the target point should be in rows
    pub target_row: usize,
    #[arg(long = "tc")]
    /// Where the target point should be in columns
    pub target_column: usize,
}
