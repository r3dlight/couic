use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::{Args, Subcommand};
use ipnet::IpNet;

use client::CouicClient;
use common::{Policy, Set, SetName};

use super::{Command, CommandError};
use crate::ripe;

#[derive(Args, Debug)]
pub struct SetsCommand {
    #[command(subcommand)]
    command: SetsSubCommand,
}

#[derive(Subcommand, Debug)]
#[command(about = "Control sets")]
enum SetsSubCommand {
    #[command(about = "List sets for a policy")]
    List {
        #[arg(help = "Policy (drop or ignore)")]
        policy: Policy,
    },
    #[command(about = "Inspect a specific set")]
    Inspect {
        #[arg(help = "Policy (drop or ignore)")]
        policy: Policy,
        #[arg(help = "Set name")]
        name: SetName,
    },
    #[command(about = "Create a new set")]
    Create {
        #[arg(help = "Policy (drop or ignore)")]
        policy: Policy,
        #[arg(help = "Set name")]
        name: SetName,
        #[arg(
            help = "CIDR entries",
            num_args = 1..,
            conflicts_with_all = ["from_asn", "from_file"],
            required_unless_present_any = ["from_asn", "from_file"]
        )]
        entries: Vec<IpNet>,
        #[arg(
            long,
            help = "Import prefixes from ASN via RIPE NCC RIPEstat (e.g., 200373 or AS200373).",
            conflicts_with_all = ["entries", "from_file"],
            required_unless_present_any = ["entries", "from_file"]
        )]
        from_asn: Option<String>,
        #[arg(
            long,
            help = "Import CIDRs from file (one per line, # for comments)",
            conflicts_with_all = ["entries", "from_asn"],
            required_unless_present_any = ["entries", "from_asn"]
        )]
        from_file: Option<PathBuf>,
    },
    #[command(about = "Update a set (replaces all entries)")]
    Update {
        #[arg(help = "Policy (drop or ignore)")]
        policy: Policy,
        #[arg(help = "Set name")]
        name: SetName,
        #[arg(help = "CIDR entries", num_args = 1..)]
        entries: Vec<IpNet>,
    },
    #[command(about = "Delete a set")]
    Delete {
        #[arg(help = "Policy (drop or ignore)")]
        policy: Policy,
        #[arg(help = "Set name")]
        name: SetName,
    },
    #[command(about = "Reload sets into eBPF maps")]
    Reload,
}

impl Command for SetsCommand {
    fn execute(&self, client: &mut CouicClient) -> Result<(), CommandError> {
        match &self.command {
            SetsSubCommand::List { policy } => {
                let sets = client.sets().list(*policy)?;
                if sets.is_empty() {
                    println!("No sets found for policy '{policy}'");
                } else {
                    for set in sets {
                        println!("{set}");
                    }
                }
            }
            SetsSubCommand::Inspect { policy, name } => {
                let set = client.sets().get(*policy, name)?;
                println!("{set}");
            }
            SetsSubCommand::Create {
                policy,
                name,
                entries,
                from_asn,
                from_file,
            } => {
                let final_entries = if let Some(asn) = from_asn {
                    println!("Fetching prefixes for ASN: {asn}");
                    let prefixes = ripe::fetch_asn_prefixes(asn)?;
                    let count = prefixes.len();
                    println!("Retrieved {count} prefixes from RIPE NCC RIPEstat");
                    prefixes
                } else if let Some(path) = from_file {
                    println!("Reading CIDRs from file: {}", path.display());
                    let prefixes = read_cidrs_from_file(path)?;
                    let count = prefixes.len();
                    println!("Loaded {count} CIDRs from file");
                    prefixes
                } else {
                    entries.clone()
                };

                let set = Set {
                    name: name.clone(),
                    entries: final_entries,
                };
                let created = client.sets().create(*policy, &set)?;
                println!(
                    "Set '{}' created with {} entries",
                    created.name,
                    created.entries.len()
                );
                println!("Note: Run 'couicctl sets reload' to apply changes");
            }
            SetsSubCommand::Update {
                policy,
                name,
                entries,
            } => {
                let set = Set {
                    name: name.clone(),
                    entries: entries.clone(),
                };
                let updated = client.sets().update(*policy, name, &set)?;
                println!(
                    "Set '{}' updated with {} entries",
                    updated.name,
                    updated.entries.len()
                );
                println!("Note: Run 'couicctl sets reload' to apply changes");
            }
            SetsSubCommand::Delete { policy, name } => {
                client.sets().delete(*policy, name)?;
                println!("Set '{name}' deleted");
                println!("Note: Run 'couicctl sets reload' to apply changes");
            }
            SetsSubCommand::Reload => {
                client.sets().reload()?;
                println!("Sets reloaded successfully");
            }
        }
        Ok(())
    }
}

fn read_cidrs_from_file(path: &PathBuf) -> Result<Vec<IpNet>, CommandError> {
    let file = fs::File::open(path).map_err(|e| {
        CommandError::Generic(format!("Failed to open file {}: {}", path.display(), e))
    })?;
    let reader = BufReader::new(file);

    let mut cidrs = Vec::new();
    let mut errors = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| {
            CommandError::Generic(format!(
                "Failed to read line {}: {}",
                line_num.saturating_add(1),
                e
            ))
        })?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        match line.parse::<IpNet>() {
            Ok(cidr) => cidrs.push(cidr),
            Err(e) => errors.push(format!(
                "line {}: {} ({})",
                line_num.saturating_add(1),
                line,
                e
            )),
        }
    }

    if !errors.is_empty() {
        return Err(CommandError::Generic(format!(
            "Failed to parse {} CIDR(s):\n{}",
            errors.len(),
            errors.join("\n")
        )));
    }

    if cidrs.is_empty() {
        return Err(CommandError::Generic(
            "No valid CIDRs found in file".to_string(),
        ));
    }

    Ok(cidrs)
}
