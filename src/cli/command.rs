use clap::{arg, command, crate_authors, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
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
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_name("depth")
                .value_parser(clap::value_parser!(usize))
                .help("Specify max set length"),
        )
        //.arg(arg!(-s --short ... "Shows a short aporism."))
        .get_matches()
}
