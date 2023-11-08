use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Command {
    Generate { path: PathBuf },
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Generate { path } => {
            fs::create_dir(path)?;
        }
    }

    Ok(())
}
