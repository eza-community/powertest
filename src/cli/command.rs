use clap::{arg, command, crate_authors, Arg, Command};

/// Parses command-line arguments using the `clap` library.
///
/// # Returns
///
/// Returns an instance of `ArgMatches` which contains the parsed arguments.
///
/// # Arguments
///
/// - `--init`: Initializes the `powertest.toml` file.
/// - `-c` or `--config <config>`: Specifies the configuration file.
/// - `-D` or `--dump <dump>`: Specifies the dump directory.
/// - `-r` or `--run <run>`: Specifies the help command.
/// - `-d` or `--depth <depth>`: Specifies the maximum set length. The value should be of type `usize`.
///
/// # Examples
///
/// ```bash
/// $ my_program --init
/// $ my_program -c my_config.toml
/// $ my_program --dump ./dumps/
/// $ my_program --run help_command
/// $ my_program --depth 5
/// ```
pub fn build_cli() -> Command {
    command!()
        .author(crate_authors!("\n"))
        .arg(arg!(--init ... "Init powertest.toml"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("config")
                .help("Specify config file"),
        )
        .arg(
            Arg::new("dump")
                .short('D')
                .long("dump")
                .value_name("dump")
                .help("Specify dump dir"),
        )
        .arg(
            Arg::new("run")
                .short('r')
                .long("run")
                .value_name("run")
                .help("Specify help command"),
        )
        .arg(arg!(--stdin ... "Get help from stdin"))
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_name("depth")
                .value_parser(clap::value_parser!(usize))
                .help("Specify max set length"),
        )
    //.arg(arg!(-s --short ... "Shows a short aporism."))
}
