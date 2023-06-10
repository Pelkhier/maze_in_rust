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
    #[arg(long)]
    /// Where the starting point should be in rows
    pub sr: usize,
    #[arg(long)]
    /// Where the starting point should be in columns
    pub sc: usize,
    #[arg(long)]
    /// Where the target point should be in rows
    pub tr: usize,
    #[arg(long)]
    /// Where the target point should be in columns
    pub tc: usize,
}
