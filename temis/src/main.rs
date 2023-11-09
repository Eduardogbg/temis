use clap::{Parser, Subcommand};
use near_workspaces;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, Write},
    path::PathBuf,
    thread,
};

use crate::transactions::{create_contract_subaccount, deploy_contract};

mod constants;
mod transactions;

const TMP_PATH: &'static str = "./tmp";

#[derive(Serialize, Deserialize)]
pub struct ContractConfig {
    crate_name: String,
    crate_path: String,
}

impl ContractConfig {
    pub fn manifest_path(&self) -> Result<PathBuf, std::io::Error> {
        fs::canonicalize(format!("{}/Cargo.toml", self.crate_path))
    }
}

#[derive(Serialize, Deserialize)]
struct SandboxConfig {
    contracts: Vec<ContractConfig>,
}

#[derive(Subcommand, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Command {
    Generate {
        input_path: PathBuf,
        output_path: PathBuf,
    },
    Sandbox {
        config_path: PathBuf,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

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

            near_abi_client::Generator::new(output_path)
                .file(file.clone())
                .generate()?;

            fs::remove_file(file)?;
            fs::remove_dir(TMP_PATH)?;
        }

        Command::Sandbox { config_path } => {
            let config_file = fs::read(config_path)?;

            let sandbox_config: SandboxConfig = serde_json::from_slice(&config_file)?;

            let mut wasms: HashMap<String, String> = HashMap::new();
            for contract in sandbox_config.contracts {
                let build_output = std::process::Command::new("cargo")
                    .args(&[
                        "build",
                        "--target=wasm32-unknown-unknown",
                        "--release",
                        "--message-format=json",
                    ])
                    .current_dir(contract.crate_path.clone())
                    .output()?;

                let manifest_path = contract.manifest_path()?;

                for line in build_output.stdout.lines() {
                    let artifact: serde_json::Value = serde_json::from_str(line?.as_str())?;

                    if let Some(artifact_manifest_path) = artifact.get("manifest_path") {
                        if artifact_manifest_path.as_str() == manifest_path.to_str() {
                            if let Some(filenames) = artifact.get("filenames") {
                                wasms.insert(
                                    contract.crate_name.clone(),
                                    filenames
                                        .as_array()
                                        .unwrap()
                                        .get(0)
                                        .unwrap()
                                        .as_str()
                                        .unwrap()
                                        .to_string(),
                                );
                            }
                        }
                    }
                }
            }

            let worker = near_workspaces::sandbox().await?;
            println!("Sandbox RPC Address: {}", worker.rpc_addr());

            let tla = worker.dev_create_account().await?;

            for (crate_name, wasm_path) in wasms.iter() {
                let contract_account = create_contract_subaccount(&tla, crate_name).await?;

                let wasm_bytes = fs::read(wasm_path)?;

                deploy_contract(&contract_account, &wasm_bytes).await?;
            }

            println!("All contracts deployed and initialized."); // TODO: pretty print contract accounts

            // hangs the thread to keep worker alive
            thread::park();
        }
    }

    Ok(())
}
