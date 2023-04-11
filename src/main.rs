mod mapping;
mod parsers;

use clap::{arg, command, Parser};
use mapping::Mapping;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short='u', long="uid")]
    /// mappings only for uid
    uid_only_mappings: Vec<Mapping>,

    #[arg(short='g', long="gid")]
    /// mappings only for gid
    gid_only_mappings: Vec<Mapping>,

    #[arg(id("both"), required(false))]
    /// mappings for both uid and gid
    mappings: Vec<Mapping>,
}

fn main() {
    let cli = Cli::parse();
}
