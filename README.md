# Rust Powerset Generator

> **Warning**
> This program is WIP

This Rust project is designed to extract command-line flags from input and generate a powerset of those flags up to a specified depth. The resulting subsets are then prefixed with a constant string and printed to the console.

## Features

    Extract short (-f) and long (--flag) command-line flags from input.
    Generate a powerset of the extracted flags up to a specified depth.
    Prefix each subset in the powerset with a constant string (eza by default).

## Usage

Compile the Rust program:

```bash
cargo build --release
```
Run the compiled binary and provide the input:

```bash
echo "-a, --all... -l, --long..." | ./target/release/your_binary_name
```

Replace `your_binary_name` with the appropriate name of your compiled binary.

The program will print the output strings, each prefixed with the constant string and containing a subset of the extracted flags.

# Configuration

    `DEPTH`: Adjust this constant to set the maximum size of the subsets in the powerset. Default is `2`.
    `BINARY`: This constant string is prefixed to each subset in the powerset. Default is `eza`.

# Dependencies

    `regex`: This crate is used for extracting flags from the input using regular expressions.

# Contribution

Feel free to fork this repository, make changes, and submit pull requests. Any contributions, whether it's improving the logic, adding features, or fixing bugs, are always welcome!
