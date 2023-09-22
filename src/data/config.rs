#![allow(clippy::type_complexity)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, write};
use std::io::Error;

use log::*;

pub const CONFIG: &str = "powertest.yaml";

const DUMP_DIR: &str = "dump";

const DEPTH: usize = 2; // Adjust this value as needed

const BINARY: &str = "eza";
const ARGS: &str = "tests/itest";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The dir to dump tests in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dump_dir: Option<String>,
    /// The maximal size of subsets in a powerset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<usize>,
    /// The binary to test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    /// Arguments to the binary we test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    /// The binary to generate from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen_binary: Option<String>,
    /// The list of commands to override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<HashMap<(Option<String>, Option<String>), Command>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    /// Command prefix, e.g. `--tree` in `--tree -L <n>`
    ///
    /// TODO: Implement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
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
            (None, Some("--time-style".to_string())),
            Command {
                prefix: None,
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
            (Some("-s".to_string()), Some("--sort".to_string())),
            Command {
                prefix: None,
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
                dump_dir: Some(DUMP_DIR.to_string()),
                depth: Some(DEPTH),
                binary: Some(BINARY.to_string()),
                args: Some(ARGS.to_string()),
                gen_binary: Some(ARGS.to_string()),
                commands: Some(commands),
            },
        }
    }
    pub fn gen_example_config(&self) -> Result<(), Error> {
        let data = serde_yaml::to_string(&self).unwrap();
        write(CONFIG, data)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(config.depth, Some(DEPTH));
        assert_eq!(config.binary, Some(BINARY.to_string()));
        assert_eq!(config.args, Some(ARGS.to_string()));
        assert!(config.commands.is_some());
    }
}
