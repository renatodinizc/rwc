# Rust `wc` Clone

A simple Rust clone of the Unix utility `wc` that counts lines, words, characters, and bytes in a given input. This project provides a command-line tool that emulates the basic functionality of the `wc` utility.

## Usage

```bash
$ cargo run -- [OPTIONS] [FILES...]
```

### Options:

- `-c`, `--bytes`: Print the byte count.
- `-m`, `--chars`: Print the character count.
- `-w`, `--words`: Print the word count.
- `-l`, `--lines`: Print the newline count.

### Files:

You can specify one or more files as arguments. Use `-` for standard input.

## Example

```bash
$ cargo run -- -lwc myfile.txt
```

This command will print the line, word, and byte counts for the file `myfile.txt`.

## Dependencies

- [clap](https://docs.rs/clap): A powerful command-line argument parser for Rust.

## How it Works

The program takes command-line arguments using `clap`, allowing users to specify options and files. It then processes the input files, either from standard input or from specified files. The counting logic is implemented, and the results are printed to the console.

## Building and Running

Ensure you have Rust installed on your system. You can build and run the project using:

```bash
$ cargo build
$ cargo run -- [OPTIONS] [FILES...]
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Rust community for their valuable contributions and support.
- [clap](https://docs.rs/clap) for providing a robust command-line parsing library in Rust.

Feel free to contribute or open issues if you find any improvements or have suggestions!