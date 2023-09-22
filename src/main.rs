use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;

mod cli;
mod data;
mod math;
mod parser;
mod utils;

fn main() -> std::io::Result<()> {
    let matches = crate::cli::parse_args();

    let mut config;

    if let Some(config_file) = matches.get_one::<String>("config") {
        config = crate::data::Config::load(config_file);
    } else {
        config = crate::data::Config::load(data::CONFIG);
    }

    if let Some(init) = matches.get_one::<u8>("init") {
        if init > &0 {
            let _ = config.gen_example_config();
            return Ok(());
        }
    }

    if let Some(dump_dir) = matches.get_one::<String>("dump") {
        config.dump_dir = Some(dump_dir.to_string());
    }

    if let Some(depth) = matches.get_one::<usize>("depth") {
        config.depth = Some(*depth);
    }

    let parse: Vec<(Option<String>, Option<String>)>;

    // This decides what binary to use:
    // 1. If user provided `run` flag, we use what they provide
    // 2. Else if the config has a `gen_binaries` record, we use that
    // 3. Else, we use stdin
    if let Some(run) = matches.get_one::<String>("run") {
        parse = match crate::utils::get_help(run, &[]) {
            Ok(parse) => crate::parser::parse(BufReader::new(parse.as_slice())),
            Err(e) => panic!("{:?}", e),
        };
    } else if let Some(run) = config.gen_binary {
        parse = match crate::utils::get_help(&run, &[]) {
            Ok(parse) => crate::parser::parse(BufReader::new(parse.as_slice())),
            Err(e) => panic!("{:?}", e),
        }
    } else {
        parse = crate::parser::parse(std::io::stdin().lock());
    }

    let mut set = vec![];

    for option in parse {
        if let Some((key, stuff)) = config.commands.as_ref().unwrap().get_key_value(&option) {
            match key {
                (Some(left), Some(right)) => {
                    if let Some(prefix) = stuff.prefix.as_ref() {
                        for value in stuff.values.as_ref().unwrap() {
                            set.push(format!("{} {} {}", prefix, left, &value));
                            set.push(format!("{} {} {}", prefix, right, &value));
                        }
                    } else {
                        for value in stuff.values.as_ref().unwrap() {
                            set.push(format!("{} {}", left, &value));
                            set.push(format!("{} {}", right, &value));
                        }
                    }
                }
                (Some(left), None) => {
                    if let Some(prefix) = stuff.prefix.as_ref() {
                        for value in stuff.values.as_ref().unwrap() {
                            set.push(format!("{} {} {}", prefix, left, &value));
                        }
                    } else {
                        for value in stuff.values.as_ref().unwrap() {
                            set.push(format!("{} {}", left, &value));
                        }
                    }
                }
                (None, Some(right)) => {
                    if let Some(prefix) = stuff.prefix.as_ref() {
                        for value in stuff.values.as_ref().unwrap() {
                            set.push(format!("{} {} {}", prefix, right, &value));
                        }
                    } else {
                        for value in stuff.values.as_ref().unwrap() {
                            set.push(format!("{} {}", right, &value));
                        }
                    }
                }
                (None, None) => todo!(),
            }
        } else {
            match &option {
                (Some(left), Some(right)) => {
                    set.push(left.to_string());
                    set.push(right.to_string());
                }
                (Some(left), None) => {
                    set.push(left.to_string());
                }
                (None, Some(right)) => {
                    set.push(right.to_string());
                }
                (None, None) => todo!(),
            }
        }
    }

    let powerset = math::generate_powerset(&set, config.depth.unwrap());

    let output_strings: Vec<String> = powerset
        .iter()
        .map(|subset| {
            format!(
                "bin.name = \"{}\"\nargs = \"{} {}\"",
                &config.binary.as_ref().unwrap(),
                &config.args.as_ref().unwrap(),
                subset.join(" ")
            )
        })
        .collect();

    println!("{output_strings:#?}");

    // Create the dump directory if it doesn't exist
    let binding = config.dump_dir.unwrap();
    let dump_path = Path::new(&binding);
    if !dump_path.exists() {
        fs::create_dir(dump_path)
            .unwrap_or_else(|_| panic!("Failed to create dump directory: {:?}", dump_path));
    }

    // Write each string in output_strings to a new file in the dump directory
    for content in &output_strings {
        // Extract the arguments part from the content
        let args_line = content.lines().nth(1).unwrap_or("");
        let args = args_line
            .split("args = \"")
            .nth(1)
            .unwrap_or("")
            .trim_end_matches('\"')
            .replace([' ', '/'], "_") // This is to handle the "tests/itest" in ARGS
            .replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..], "_"); // Sanitize for Windows

        let file_path = dump_path.join(format!("ptest_{}.toml", args));
        let mut file =
            File::create(&file_path).expect(&format!("Failed to create file at {:?}", file_path));
        let mut file = File::create(&file_path)
            .unwrap_or_else(|_| panic!("Failed to create file at {:?}", file_path));
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }

    Ok(())
}
