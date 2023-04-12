mod externals;
mod mappings;

use clap::{arg, command, Parser};
use mappings::{Mapping, Mappings};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(after_long_help = "\
Mappings can be provided in the following format:

    CT_ID_START[-CT_ID_END][:HOST_ID_START[-HOST_ID_END]]

CT_ID_START and CT_ID_END are the UID/GID range start and end from the container's view.
If CT_ID_END is not provided it defaults to the same value as CT_ID_START.
HOST_ID_START and HOST_ID_END are the UID/GID range start and end from the host's view.
If HOST_ID_END is not provided it defaults to HOST_ID_START + CT_ID_END - CT_ID_START

All range bounds are inclusive.
")]
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
        eprintln!("Mappings provided");
        if !uid_mappings.is_empty() {
            eprintln!("  UID: {}", uid_mappings);
        }
        if !gid_mappings.is_empty() {
            eprintln!("  GID: {}", gid_mappings);
        }
    }

    let host_start = 100000;
    let uid_mappings = uid_mappings.with_missing(host_start);
    let gid_mappings = gid_mappings.with_missing(host_start);

    if cli.debug {
        eprintln!("\nMappings calculated");
        if !uid_mappings.is_empty() {
            eprintln!("  UID: {}", uid_mappings);
        }
        if !gid_mappings.is_empty() {
            eprintln!("  GID: {}", gid_mappings);
        }
    }

    println!(
        "\n\
         # ct.conf"
    );
    if !uid_mappings.is_empty() {
        println!("# UID mappings");
        for m in uid_mappings.iter() {
            println!("lxc.idmap = u {} {} {}", m.ct_start, m.host_start, m.count);
        }
    }
    if !gid_mappings.is_empty() {
        println!("# GID mappings");
        for m in gid_mappings.iter() {
            println!("lxc.idmap = g {} {} {}", m.ct_start, m.host_start, m.count);
        }
    }

    let current_user_uid = externals::geteuid();
    if !uid_mappings.is_empty() {
        println!(
            "\n\
             #/etc/subuid"
        );
        println!("{current_user_uid}:{host_start}:65536");
        for m in uid_mappings.iter().filter(|m| m.host_start < host_start) {
            println!("{current_user_uid}:{}:{}", m.host_start, m.count);
        }
    }
    if !gid_mappings.is_empty() {
        println!(
            "\n\
             #/etc/subgid"
        );
        println!("{current_user_uid}:{host_start}:65536");
        for m in gid_mappings.iter().filter(|m| m.host_start < host_start) {
            println!("{current_user_uid}:{}:{}", m.host_start, m.count);
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
