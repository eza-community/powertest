// TODO: remove allows
#[allow(unused)]
use std::fs::File;
#[allow(unused)]
use std::io::Write;
#[allow(unused)]
use std::path::Path;

const CONFIG: &'static str = ".ptest.yaml";

pub mod data {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
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
        pub commands: Option<HashMap<(Option<String>, Option<String>), Command>>,
    }

    /// TODO: Redundant data, key and value already act as short and long
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
            let mut commands = HashMap::new();
            commands.insert(
                (None, Some("time-style".to_string())),
                Command {
                    short: None,
                    long: Some("time-style".to_string()),
                    values: Some(vec![
                        "default".to_string(),
                        "iso".to_string(),
                        "long-iso".to_string(),
                        "full-iso".to_string(),
                        "relative".to_string(),
                    ]),
                },
            );
            commands.insert(
                (Some("-s".to_string()), Some("sort".to_string())),
                Command {
                    short: Some("-s".to_string()),
                    long: Some("time-style".to_string()),
                    // TODO: non-exhaustive
                    values: Some(vec![
                        "accessed".to_string(),
                        "age".to_string(),
                        "changed".to_string(),
                        "created".to_string(),
                        "date".to_string(),
                    ]),
                },
            );
            match Self::new(path) {
                Ok(config) => config,
                Err(_) => Config {
                    depth: Some(DEPTH),
                    binary: Some(BINARY.to_string()),
                    args: Some(ARGS.to_string()),
                    commands: Some(commands),
                },
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::data::*;
        use std::fs::File;
        use std::io::Write;

        const TEST_CONFIG: &str = r#"
depth: 3
binary: "test_binary"
args: "test_args"
"#;

        #[test]
        fn test_config_new() {
            // Create a temporary YAML file
            let path = "temp_test_config.yaml";
            let mut file = File::create(path).unwrap();
            file.write_all(TEST_CONFIG.as_bytes()).unwrap();

            // Load the configuration
            let config = Config::new(path).unwrap();

            // Clean up
            std::fs::remove_file(path).unwrap();

            // Assertions
            assert_eq!(config.depth, Some(3));
            assert_eq!(config.binary, Some("test_binary".to_string()));
            assert_eq!(config.args, Some("test_args".to_string()));
        }

        #[test]
        fn test_config_load_with_invalid_path() {
            let config = Config::load("non_existent_path.yaml");

            // Assertions for default values
            assert_eq!(config.depth, Some(crate::data::DEPTH));
            assert_eq!(config.binary, Some(crate::data::BINARY.to_string()));
            assert_eq!(config.args, Some(crate::data::ARGS.to_string()));
            assert!(config.commands.is_some());
        }
    }
}

pub mod parser {
    use regex::Regex;

    use std::io::{self, BufRead};

    pub fn parse() -> Vec<(Option<String>, Option<String>)> {
        let pattern = r"(-[^-])(, )(--\w+)";
        let re = Regex::new(pattern).unwrap();

        let mut set = vec![];

        // Read lines from stdin until EOF
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let line = line.unwrap().trim().to_string();

            // Check if the line matches the combined long
            for capture in re.captures_iter(&line) {
                // println!("{:?}, {:?}", capture.get(1), capture.get(3));
                set.push(match (capture.get(1), capture.get(3)) {
                    (Some(short), Some(long)) => (
                        Some(short.as_str().to_string()),
                        Some(long.as_str().to_string()),
                    ),
                    (Some(short), None) => (Some(short.as_str().to_string()), None),
                    (None, Some(long)) => (None, Some(long.as_str().to_string())),
                    (None, None) => (None, None),
                })
            }
        }

        set
    }
}

fn main() -> std::io::Result<()> {
    let config = crate::data::Config::load(CONFIG);

    // println!("{config:#?}");

    let set = crate::parser::parse();

    // println!("{set:#?}");

    let powerset = math::generate_powerset_combined(&set, config.depth.unwrap());

    // println!("{powerset:#?}");

    // let output_strings: Vec<(Option<String>, Option<String>)> = powerset.iter().collect();
    // .map(|subset| {
    //     format!(
    //         "bin.name = \"{}\"\nargs = \"{} {}\"",
    //         &config.binary.as_ref().unwrap(),
    //         &config.args.as_ref().unwrap(),
    //         subset.join(" ")
    //     )
    // })

    // Go through all elements in the powerset
    // println!("{:#?}", powerset);
    for option_pair in powerset {
        for thing in &option_pair {
            let stuff = config.commands.as_ref().expect("NOT_FOUND").get(&thing);
            // println!("{thing:?}");
            // println!("{stuff:#?}");
        }
        println!("{:#?}", option_pair);
    }

    // println!("{config:#?}");

    /*
     *
    println!("{output_strings:#?}");

    // println!("Output Strings: {:#?}", output_strings);


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
    }*/

    Ok(())
}

pub mod math {
    pub fn generate_powerset<T: Clone>(set: &[T], depth: usize) -> Vec<Vec<T>> {
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
    pub fn generate_powerset_combined(
        set: &[(Option<String>, Option<String>)],
        depth: usize,
    ) -> Vec<Vec<(Option<String>, Option<String>)>> {
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
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_generate_powerset() {
            let set: Vec<i32> = vec![1, 2, 3];
            let depth = 2;
            let mut result = generate_powerset(&set, depth);
            let mut expected = vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
            ];
            // NOTE: Order isn't guaranteed, so we do this.
            result.sort();
            expected.sort();

            assert_eq!(result, expected);
        }

        #[test]
        fn test_generate_powerset_empty() {
            let set: Vec<i32> = vec![];
            let depth = 2;
            let result = generate_powerset(&set, depth);
            let expected: Vec<Vec<i32>> = vec![vec![]];
            assert_eq!(result, expected);
        }

        #[test]
        fn test_generate_powerset_combined() {
            let set = vec![
                (Some("a".to_string()), Some("b".to_string())),
                (Some("c".to_string()), None),
                (None, Some("d".to_string())),
                (None, None),
            ];
            let depth = 2;
            let mut result = generate_powerset_combined(&set, depth);
            let mut expected = vec![
                vec![],
                vec![(Some("a".to_string()), Some("b".to_string()))],
                vec![(Some("c".to_string()), None)],
                vec![(None, Some("d".to_string()))],
                vec![(None, None)],
                vec![
                    (Some("a".to_string()), Some("b".to_string())),
                    (Some("c".to_string()), None),
                ],
                vec![
                    (Some("a".to_string()), Some("b".to_string())),
                    (None, Some("d".to_string())),
                ],
                vec![(Some("a".to_string()), Some("b".to_string())), (None, None)],
                vec![(Some("c".to_string()), None), (None, Some("d".to_string()))],
                vec![(Some("c".to_string()), None), (None, None)],
                vec![(None, Some("d".to_string())), (None, None)],
            ];

            // NOTE: Order isn't guaranteed, so we do this.
            result.sort();
            expected.sort();

            assert_eq!(result, expected);
        }

        #[test]
        fn test_generate_powerset_combined_empty() {
            let set: Vec<(Option<String>, Option<String>)> = vec![];
            let depth = 2;
            let result = generate_powerset_combined(&set, depth);
            let expected: Vec<Vec<(Option<String>, Option<String>)>> = vec![vec![]];
            assert_eq!(result, expected);
        }
    }
}
