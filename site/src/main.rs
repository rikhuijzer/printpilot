use anyhow::Result;
use clap::command;
use clap::Parser;
use clap::Subcommand;

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

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.command {
        Command::Generate(project) => match project {
            Project::Site => printpilot::html::generate_site(),
        },
    }?;

    Ok(())
}
