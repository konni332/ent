use clap::{Parser, ValueEnum};



#[derive(Parser, Debug, Clone, Default)]
pub struct Cli {

    /// Path to the directory to search. Defaults to the current working directory
    pub path: Option<String>,

    /// Maximum depth to search
    #[clap(short, long)]
    pub depth: Option<usize>,

    /// Show all
    #[clap(short, long, conflicts_with_all = &["dirs_only", "files_only"])]
    pub all: bool,

    /// Show only directories
    #[clap(short = 'D', long = "dirs-only")]
    pub dirs_only: bool,

    /// Show only files
    #[clap(short = 'F', long = "files-only")]
    pub files_only: bool,

    /// Show ignored files
    #[clap(short, long)]
    pub ignored: bool,

    /// Show hidden files and directories in tree
    #[clap(short = 'H', long)]
    pub hidden: bool,

    /// Export tree as a file
    #[clap(short, long, value_enum)]
    pub export: Option<Format>,
}

#[derive(Parser, Debug, Clone, ValueEnum)]
pub enum Format {
    Json,
}