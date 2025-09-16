
use anyhow::Result;

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

pub fn generate_site() -> Result<()> {
    let html = "
        <!DOCTYPE html>
        <html lang='en'>

        <head>
        <title>PrintPilot.org</title>
        <meta charset='utf-8'>
        <meta name='viewport' content='width=device-width'>
        </head>

        <body>
            <b>hello 2</b>
        </body>
        </html>
    ";
    write_html(html, "index.html")
}
