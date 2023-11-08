use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Command {
    Generate,
}

impl Command {
    fn as_str(&self) -> &'static str {
        match self {
            Command::Generate => "generate",
        }
    }
}

#[derive(Parser, Debug, Clone)]
struct Args {
    command: Command,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Generate => {}
    }

    println!("Hello, {}!", args.command.as_str());
}
