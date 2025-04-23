use luau_formatter::{Config, format_with_config};
use luau_parser::parser::Parser;
use std::{fs, io, path::Path};

#[inline]
fn format_folder(path: &Path, config: &Config) -> io::Result<()> {
    if path.file_name().is_some_and(|name| name == ".git") {
        return Ok(());
    }

    for file in path.read_dir()? {
        format_path(file?.path(), config)?;
    }

    Ok(())
}

fn format_file(path: &Path, config: &Config) -> io::Result<()> {
    if !path
        .extension()
        .is_some_and(|extension| extension == "lua" || extension == "luau")
    {
        return Ok(());
    }

    let content = fs::read_to_string(path)?;
    let mut parser = Parser::new(&content);
    let cst = parser.parse(path.to_string_lossy().as_ref());

    if let Ok(code) = format_with_config(&cst, config) {
        fs::write(path, code.as_bytes())?;
    } else {
        eprintln!("{:?} has syntax errors. Skipping.", path);
    }

    Ok(())
}

#[inline]
pub fn format_path<P: AsRef<Path>>(path: P, config: &Config) -> io::Result<()> {
    let path = path.as_ref();

    if path.is_dir() {
        format_folder(path, config)
    } else {
        format_file(path, config)
    }
}
