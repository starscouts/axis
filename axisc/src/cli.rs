use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    pub source: PathBuf,

    /// Set a custom output path
    #[arg(short, long, value_name = "output_path")]
    pub output: Option<PathBuf>,
}
