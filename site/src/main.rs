use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use clap::command;

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
enum Project {
    Site,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
enum Command {
    #[command(subcommand)]
    Generate(Project),
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.command {
        Command::Generate(project) => match project {
            Project::Site => site::html::generate_site(),
        },
    }?;

    Ok(())
}
