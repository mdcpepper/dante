use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use lightningcss::{
    bundler::{Bundler, FileProvider},
    stylesheet::{ParserOptions, PrinterOptions},
};

#[derive(Debug, Parser)]
#[command(
    name = "lattice-css-build",
    about = "Bundle and minify CSS using Lightning CSS"
)]
struct Cli {
    /// CSS entry file (can use @import).
    #[arg(long)]
    input: PathBuf,

    /// Main output path.
    #[arg(long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    build_css(&cli)
}

fn build_css(config: &Cli) -> Result<()> {
    let minify = matches!(env::var("TRUNK_PROFILE").as_deref(), Ok("release"));
    let file_provider = FileProvider::new();

    let mut bundler = Bundler::new(&file_provider, None, ParserOptions::default());

    let stylesheet = bundler
        .bundle(&config.input)
        .map_err(|error| anyhow!("failed to bundle {}: {error}", display(&config.input)))?;

    let css = stylesheet
        .to_css(PrinterOptions {
            minify,
            ..PrinterOptions::default()
        })
        .context("failed to print CSS")?
        .code;

    write_css_if_changed(&config.output, &css)?;

    Ok(())
}

fn write_css_if_changed(path: &PathBuf, css: &str) -> Result<()> {
    if file_content_is(path, css)? {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", display(parent)))?;
    }

    fs::write(path, css).with_context(|| format!("failed to write {}", display(path)))?;

    Ok(())
}

fn file_content_is(path: &PathBuf, expected: &str) -> Result<bool> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content == expected),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error).with_context(|| format!("failed to read {}", display(path))),
    }
}

fn display(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
