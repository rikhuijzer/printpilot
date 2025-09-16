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

fn format_html(html: &str) -> String {
    let document = scraper::Html::parse_document(html);
    document.html()
}

fn write_html(html: &str, filename: &str) -> Result<()> {
    let html = format_html(html);
    let path = std::path::Path::new("_public").join(filename);
    std::fs::write(path, html)?;
    Ok(())
}

fn generate_site() -> Result<()> {
    let html = "
        <!DOCTYPE html>
        <html lang='en'>

        <head>
        <title>PrintPilot.org</title>
        <meta charset='utf-8'>
        <meta name='viewport' content='width=device-width'>
        </head>

        <body>
            <b>hello</b>
        </body>
        </html>
    ";
    write_html(html, "index.html")
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.command {
        Command::Generate(project) => match project {
            Project::Site => generate_site(),
        },
    }?;

    Ok(())
}
