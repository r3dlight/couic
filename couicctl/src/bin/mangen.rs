use std::io::Result;
use std::path::PathBuf;
use std::{env, fs};

use clap::{Command, CommandFactory};
use clap_mangen::Man;
use couicctl::cli::Cli;

fn main() -> Result<()> {
    let out_dir =
        env::var("OUT_DIR").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let root_cmd = Cli::command();
    generate_manpages(&root_cmd, &PathBuf::from(&out_dir), &[])?;
    Ok(())
}

fn generate_manpages(cmd: &Command, out_dir: &PathBuf, parent_cmds: &[String]) -> Result<()> {
    let mut cmd_path = parent_cmds.to_owned();
    cmd_path.push(cmd.get_name().to_string());

    // Join command path with '-' for filename, e.g. "couicctl-drop.1"
    let filename = format!("{}.1", cmd_path.join("-"));
    let out_path = out_dir.join(filename);

    let man = Man::new(cmd.clone());
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;
    fs::write(&out_path, buffer)?;
    println!("Man page generated at {}", out_path.display());

    // Recurse for subcommands
    for sub in cmd.get_subcommands() {
        generate_manpages(sub, out_dir, &cmd_path)?;
    }
    Ok(())
}
