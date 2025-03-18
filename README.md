# Extractorb

A simple CLI tool that extracts text from documents and outputs it to a text file.

## Features

- Extracts text from various document formats (PDF, Word, Excel, PowerPoint, etc.)
- Supports both local files and URLs as input
- Outputs plain text to a specified file (defaults to output.txt)
- Built on the fast and efficient [extractous](https://github.com/yobix-ai/extractous) library
- install tesseract seperately if you want OCR as well

### Prerequisites

Make sure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/).


### Using in goose

Ask goose: 
"please use this program https://github.com/michaelneale/extractorb/releases/download/latest/extractorb to process PDFs from here on."
