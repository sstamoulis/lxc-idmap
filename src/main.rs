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
    let uid_mappings = {
        let mut m = cli.uid_only_mappings;
        m.extend(cli.mappings.clone());
        Mappings::from(m)
    };
    let gid_mappings = {
        let mut m = cli.gid_only_mappings;
        m.extend(cli.mappings);
        Mappings::from(m)
    };

    if cli.debug {
        eprintln!("Mappings\n");
        if !uid_mappings.is_empty() {
            eprintln!("  UID: {}", uid_mappings);
        }
        if !gid_mappings.is_empty() {
            eprintln!("  GID: {}", gid_mappings);
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
