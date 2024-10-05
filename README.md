# BCut2SRT

BCut2SRT is a command-line tool that converts BCut JSON files into SRT (SubRip Subtitle) format. This tool parses subtitle information from BCut project files and outputs them as an SRT file, commonly used for subtitles in videos.

## Features

- Parse BCut JSON data containing subtitle information.
- Extract subtitle clips and generate SRT subtitle files.
- Easy-to-use command-line interface with input and output file options.

## Prerequisites

Before compiling or running the program, you need to have the following tools installed:

- Rust

You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Compilation

To compile the program, follow these steps:

1. Clone this repository:

   ```sh
   git clone https://github.com/Akiko97/bcut2srt.git
   cd bcut2srt
   ```

2. Compile the program using Cargo:

   ```sh
   cargo build --release
   ```

   This will generate an optimized executable in the `target/release` directory.

## Usage

Once the program is compiled, you can use it by running the executable from the command line. The program takes two required arguments: an input BCut JSON file and an output SRT file.

### Command-line Options

- `-i, --input <file>`: The input BCut JSON file.
- `-o, --output <file>`: The output SRT file.

### Example Usage

```sh
./target/release/bcut2srt -i input_file.json -o output_file.srt
```

In this example, `input_file.json` is the BCut JSON file you want to convert, and `output_file.srt` is the file where the converted SRT subtitles will be saved.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Author

GitHub@Akiko97 (mud.miscue_0l@icloud.com)
