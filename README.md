# Extractorb

A simple CLI tool that extracts text from documents and outputs it to a text file.

## Features

- Extracts text from various document formats (PDF, Word, Excel, PowerPoint, etc.)
- Supports both local files and URLs as input
- Outputs plain text to a specified file (defaults to output.txt)
- Built on the fast and efficient [extractous](https://github.com/yobix-ai/extractous) library

## Installation

### Prerequisites

Make sure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/).

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/extractorb.git
cd extractorb

# Build the project
cargo build --release

# The binary will be available at target/release/extractorb
```

## Usage

```bash
# Extract text from a local file (output to output.txt by default)
extractorb path/to/document.pdf

# Extract text from a URL
extractorb https://example.com/document.pdf

# Specify a custom output file
extractorb path/to/document.pdf -o extracted_text.txt
extractorb path/to/document.pdf --output extracted_text.txt
```

### Command-line options

```
Usage: extractorb [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file path or URL to extract text from

Options:
  -o, --output <OUTPUT>  Output file path [default: output.txt]
  -h, --help            Print help
  -V, --version         Print version
```

## Supported File Formats

Extractorb supports all file formats that the extractous library can handle, including:
- PDF documents
- Microsoft Office documents (Word, Excel, PowerPoint)
- HTML files
- Plain text files
- And many more

## License

This project is licensed under the Apache License 2.0.