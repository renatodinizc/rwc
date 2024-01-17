# Rust WC (Word Count) Utility

A simple command-line utility written in Rust that mimics the functionality of the classic GNU `wc` command. The utility provides a count of lines, words, characters, and bytes in one or more files or standard input.

## Usage
To use this utility, simply run the executable and specify the target file(s) as command-line arguments. By default, the utility will count lines, words, characters, and bytes. Additionally, various options are available to customize the output.

### Command-line Options
- **-l, --lines:** Print the newline count.
- **-w, --words:** Print the word count.
- **-c, --bytes:** Print the byte count.
- **-m, --chars:** Print the character count.

### Example Usage
```bash
$ ./target/release/rwc -lwm file1.txt file2.txt
```

This example will count the lines, words, and characters in the specified files.

## Dependencies

- [clap](https://docs.rs/clap): A powerful command-line argument parser for Rust.

## How it Works

The program takes command-line arguments using `clap`, allowing users to specify options and files. It then processes the input files, either from standard input or from specified files. The counting logic is implemented, and the results are printed to the console.

## Build and Run

To build the project, ensure you have Rust installed and run the following command:

```bash
$ cargo build --release
```

To run the utility, execute the generated binary:

```bash
$ ./target/release/rwc file1.txt file2.txt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgments

- The Rust community for their valuable contributions and support.
- [clap](https://docs.rs/clap) for providing a robust command-line parsing library in Rust.

Feel free to contribute or open issues if you find any improvements or have suggestions!