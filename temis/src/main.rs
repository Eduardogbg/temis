use std::{fs, io::Write, path::PathBuf};

use clap::{Parser, Subcommand};

const TMP_PATH: &'static str = "./tmp";

#[derive(Subcommand, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Command {
    Generate {
        input_path: PathBuf,
        output_path: PathBuf,
    },
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn find_file_in_dir(dir_path: &str) -> std::io::Result<String> {
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            return Ok(path.to_string_lossy().into_owned());
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No file found",
    ))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("asdas");

    match args.command {
        Command::Generate {
            input_path,
            output_path,
        } => {
            fs::create_dir(TMP_PATH)?;

            std::process::Command::new("cargo")
                .args(&[
                    "near",
                    "abi",
                    "--manifest-path",
                    format!("{}/Cargo.toml", input_path.to_string_lossy()).as_str(),
                    "--out-dir",
                    "./tmp",
                ])
                .status()?;

            let file = find_file_in_dir(TMP_PATH)?;
            println!("file path {}", file);

            let abi_json = String::from_utf8(fs::read(file.clone())?)?;

            let asdasd = jq_rs::run(
                "walk(if type == \"object\" and has(\"minimum\") then del(.minimum) else . end)",
                abi_json.as_str(),
            )
            .unwrap();

            let sanitized = jq_rs::run(
                "walk(if type == \"object\" then with_entries(if .key == \"Promise\" then .value |= {} else . end) else . end)",
                asdasd.as_str()
            ).unwrap();

            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file.clone())?;
            f.write_all(sanitized.as_bytes())?;
            f.flush()?;

            println!("chegou aq");

            near_abi_client::Generator::new(output_path)
                .file(file.clone())
                .generate()?;

            fs::remove_file(file)?;
            fs::remove_dir(TMP_PATH)?;
        }
    }

    Ok(())
}
