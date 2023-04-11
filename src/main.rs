mod mappings;

use clap::{arg, command, Parser};
use mappings::{Mapping, Mappings};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'u', long = "uid")]
    /// Mappings only for uid
    uid_only_mappings: Vec<Mapping>,

    #[arg(short = 'g', long = "gid")]
    /// Mappings only for gid
    gid_only_mappings: Vec<Mapping>,

    #[arg()]
    /// Mappings for both uid and gid
    mappings: Vec<Mapping>,

    #[arg(long)]
    /// Print debug messages in stderr
    debug: bool,
}

fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            err.exit();
        }
    };
    let uid_only_mappings: Mappings = cli.uid_only_mappings.into();
    let gid_only_mappings: Mappings = cli.gid_only_mappings.into();
    let mappings: Mappings = cli.mappings.into();

    if cli.debug {
        eprintln!("Mappings\n");
        if !uid_only_mappings.is_empty() {
            eprintln!("  UID: {}", uid_only_mappings);
        }
        if !gid_only_mappings.is_empty() {
            eprintln!("  GID: {}", gid_only_mappings);
        }
        if !mappings.is_empty() {
            eprintln!("  UID/GID: {}", mappings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_mapping() -> Result<(), String> {
        match Cli::try_parse_from(["", "1005:1006"]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
