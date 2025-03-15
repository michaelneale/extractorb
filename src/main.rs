use anyhow::{Context, Result};
use clap::Parser;
use extractous::Extractor;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

/// A CLI tool for extracting text from documents using extractous library
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path or URL to extract text from
    #[arg(required = true)]
    input: String,

    /// Output file path (defaults to output.txt in current directory)
    #[arg(short, long, default_value = "output.txt")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Create a new extractor
    let extractor = Extractor::new();
    
    // Extract content based on whether input is a URL or file path
    let (content, metadata) = if is_url(&args.input) {
        println!("Extracting text from URL: {}", args.input);
        extractor
            .extract_url_to_string(&args.input)
            .context("Failed to extract text from URL")?
    } else {
        println!("Extracting text from file: {}", args.input);
        extractor
            .extract_file_to_string(&args.input)
            .context("Failed to extract text from file")?
    };
    
    // Write content to output file
    let mut file = File::create(&args.output)
        .context(format!("Failed to create output file: {:?}", args.output))?;
    
    file.write_all(content.as_bytes())
        .context("Failed to write content to output file")?;
    
    println!("Successfully extracted text to: {:?}", args.output);
    println!("Metadata: {:?}", metadata);
    
    Ok(())
}

/// Check if the input string is a valid URL
fn is_url(input: &str) -> bool {
    Url::parse(input).is_ok()
}