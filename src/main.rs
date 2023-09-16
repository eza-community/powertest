use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

const CONFIG: &'static str = ".ptest.yaml";

pub mod data {
    use serde::{Deserialize, Serialize};
    use std::fs;

    use log::*;

    const DEPTH: usize = 2; // Adjust this value as needed

    const BINARY: &'static str = "eza";
    const ARGS: &'static str = "tests/itest";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub depth: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub binary: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub args: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub commands: Option<Vec<Command>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Command {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub short: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub long: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub values: Option<Vec<String>>,
    }

    impl Config {
        /// Loads the configuration toml from a path into the Config struct.
        pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
            debug!("initializing new Config struct");

            let yaml = fs::read_to_string(path)?;
            debug!("deserialized yaml from config file");

            let config = serde_yaml::from_str(&yaml)?;

            Ok(config)
        }
        pub fn load(path: &str) -> Self {
            match Self::new(path) {
                Ok(config) => config,
                Err(_) => Config {
                    depth: Some(DEPTH),
                    binary: Some(BINARY.to_string()),
                    args: Some(ARGS.to_string()),
                    commands: Some(vec![Command {
                        short: None,
                        long: Some("time-style".to_string()),
                        values: Some(vec![
                            "default".to_string(),
                            "iso".to_string(),
                            "long-iso".to_string(),
                            "full-iso".to_string(),
                            "relative".to_string(),
                        ]),
                    }]),
                },
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    use crate::data::Config;

    let config = Config::load(CONFIG);

    println!("{config:#?}");

    let short = r"(-[^-])";
    let long = r"(--\w+)";
    let re_short = Regex::new(short).unwrap();
    let re_long = Regex::new(long).unwrap();

    let mut set = vec![];

    // Read lines from stdin until EOF
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap().trim().to_string();

        // Check if the line matches the regex short
        for capture in re_short.captures_iter(&line) {
            if let Some(matched) = capture.get(1) {
                set.push(matched.as_str().to_string());
            }
        }

        // Check if the line matches the regex long
        for capture in re_long.captures_iter(&line) {
            if let Some(matched) = capture.get(1) {
                set.push(matched.as_str().to_string());
            }
        }
    }

    let powerset = generate_powerset(&set, config.depth.unwrap());

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

    println!("Output Strings: {:#?}", output_strings);

    // Create the dump directory if it doesn't exist
    let dump_path = Path::new("dump");
    if !dump_path.exists() {
        fs::create_dir(dump_path).expect("Failed to create dump directory");
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
            .replace(" ", "_")
            .replace("/", "_"); // This is to handle the "tests/itest" in ARGS

        let file_path = dump_path.join(format!("ptest_{}.toml", args));
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }

    Ok(())
}

fn generate_powerset<T: Clone>(set: &[T], depth: usize) -> Vec<Vec<T>> {
    let mut powerset = vec![vec![]];

    for item in set.iter() {
        let current_len = powerset.len();
        for i in 0..current_len {
            let mut new_subset = powerset[i].clone();
            new_subset.push(item.clone());
            if new_subset.len() <= depth {
                powerset.push(new_subset);
            }
        }
    }

    powerset
}
