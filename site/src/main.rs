use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use clap::command;
use std::os::unix::fs::PermissionsExt;

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

fn write_public(src: &[u8], filename: &str) -> Result<()> {
    let path = std::path::Path::new("_public").join(filename);

    // Set write permissions before writing.
    let path_obj = std::path::Path::new("_public").join(filename);
    if path_obj.exists() {
        let perms = std::fs::Permissions::from_mode(0o666);
        std::fs::set_permissions(&path_obj, perms)?;
    }

    std::fs::write(path, src)?;

    // Set to read-only to avoid accidental manual edits.
    let path_obj = std::path::Path::new("_public").join(filename);
    let perms = std::fs::Permissions::from_mode(0o444);
    std::fs::set_permissions(&path_obj, perms)?;
    Ok(())
}

fn write_static(filename: &str) -> Result<()> {
    let path = std::path::Path::new("site/src/static").join(filename);
    let src = std::fs::read(path)?;
    write_public(&src, filename)
}

fn generate_site() -> Result<()> {
    write_static("index.html")?;
    write_static("style.css")?;
    write_static("defer.js")?;
    write_static("nodefer.js")?;
    write_static("bushido.pdf")
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.command {
        Command::Generate(project) => match project {
            Project::Site => generate_site(),
        },
    }?;

    Ok(())
}
