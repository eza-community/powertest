# Rust Powerset Generator

> **Warning**
> This program is WIP

Powerset is a simple Rust binary (and perhaps future library), that takes a
configuration file specifying commands and uses it to produce a powerset of
those commands. This helps easily generate tests from a single source of truth,
allowing your tests to be initialized at any time from the declarative
configuration.

Historically, it would parse the output of a binary via a regex. Currently,
it's being refactored to generate configurations from this parsing, but
primarily being dependent on the actual configuration first and foremost.

This program was created for eza, but eventually, it should be usable more
broadly.

## Usage

Compile the Rust program:

```bash
cargo build --release
```

# Contribution

Feel free to fork this repository, make changes, and submit pull requests. Any contributions, whether it's improving the logic, adding features, or fixing bugs, are always welcome!
