use anyhow::Result;
use std::os::unix::fs::PermissionsExt;

fn write_public(src: &str, filename: &str) -> Result<()> {
    let path = std::path::Path::new("_public").join(filename);

    // Set write permissions before writing.
    let path_obj = std::path::Path::new("_public").join(filename);
    let perms = std::fs::Permissions::from_mode(0o666);
    std::fs::set_permissions(&path_obj, perms)?;

    std::fs::write(path, src)?;

    // Set to read-only to avoid accidental manual edits.
    let path_obj = std::path::Path::new("_public").join(filename);
    let perms = std::fs::Permissions::from_mode(0o444);
    std::fs::set_permissions(&path_obj, perms)?;
    Ok(())
}

pub fn generate_site() -> Result<()> {
    let src = "
        <!DOCTYPE html>
        <html lang='en'>

        <head>
        <title>PrintPilot.org</title>
        <meta charset='utf-8'>
        <meta name='viewport' content='width=device-width'>
        <script src='script.js'></script>
        </head>

        <body>
            <b>hello 2</b>
        </body>
        </html>
    ";
    write_public(src, "index.html")?;

    let script_src = std::fs::read_to_string("site/src/static/script.js")?;
    write_public(&script_src, "script.js")
}
