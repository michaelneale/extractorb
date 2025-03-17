Extractorb
A simple CLI tool that extracts text from documents and outputs it to a text file.

Features
Extracts text from various document formats (PDF, Word, Excel, PowerPoint, etc.)
Supports both local files and URLs as input
Outputs plain text to a specified file (defaults to output.txt)
Built on the fast and efficient extractous library
Installation
Prerequisites
Make sure you have Rust and Cargo installed. If not, install them from rustup.rs.

Download pre-built binary
You can download the latest pre-built binary for macOS from the GitHub releases:

# Download the zip file
curl -LO https://github.com/michaelneale/extractorb/releases/download/latest/extractorb-macos.zip

# Extract the binary
unzip extractorb-macos.zip

# Make it executable and move to a directory in your PATH (optional)
chmod +x extractorb
sudo mv extractorb /usr/local/bin/
Building from source
# Clone the repository
git clone https://github.com/michaelneale/extractorb.git
cd extractorb

# Build the project
cargo build --release

# The binary will be available at target/release/extractorb
Usage
# Extract text from a local file (output to output.txt by default)
extractorb path/to/document.pdf

# Extract text from a URL
extractorb https://example.com/document.pdf

# Specify a custom output file
extractorb path/to/document.pdf -o extracted_text.txt
extractorb path/to/document.pdf --output extracted_text.txt

# Show detailed metadata
extractorb path/to/document.pdf --verbose
Command-line options
Usage: extractorb [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file path or URL to extract text from

Options:
  -o, --output <OUTPUT>  Output file path [default: output.txt]
  -v, --verbose          Show detailed metadata after extraction
  -h, --help             Print help
  -V, --version          Print version
Supported File Formats
Extractorb supports all file formats that the extractous library can handle, including:

PDF documents
Microsoft Office documents (Word, Excel, PowerPoint)
HTML files
Plain text files
And many more
License
This project is licensed under the Apache License 2.0.

Continuous Integration and Deployment
This project uses GitHub Actions for continuous integration and deployment:

Every push to the main branch triggers a build and release process
The latest build is automatically published to the GitHub Releases page with the tag latest
Pre-built binaries are available for macOS
The CI/CD workflow:

Builds the project using the latest stable Rust toolchain
Creates a zip file containing the binary
Generates a SHA256 checksum for verification
Publishes these artifacts to GitHub Releases
You can also manually trigger a build and release from the GitHub Actions tab.
