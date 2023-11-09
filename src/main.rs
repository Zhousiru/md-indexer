mod document;
mod file;
mod model;
mod page;
mod util;

use std::error::Error;

use clap::Parser;
use document::DocumentOptions;
use file::{generate_pages_from_path, write_pages_to_path};

/// An opinionated static API generator for Markdown documents
#[derive(Parser)]
struct Cli {
    /// The input base path of documents
    #[arg(short, long)]
    input: String,
    /// The output base path of index files
    #[arg(short, long)]
    output: String,
    /// Maximum number of documents in a single page file
    #[arg(long)]
    size: usize,
    /// Include document content in index files
    #[arg(long)]
    with_content: bool,
    /// Include document summary (marked with `<!--more-->`) in index files
    #[arg(long)]
    with_summary: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let doc_options = &DocumentOptions {
        with_content: cli.with_content,
        with_summary: cli.with_summary,
    };

    let pages = generate_pages_from_path("/", &cli.input, doc_options)?;
    write_pages_to_path(&pages, &cli.output, cli.size)?;

    Ok(())
}
