use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

const DEPTH: usize = 2; // Adjust this value as needed
const BINARY: &'static str = "eza";

fn main() {
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

    let powerset = generate_powerset(&set, DEPTH);

    let output_strings: Vec<String> = powerset
        .iter()
        .map(|subset| format!("bin.name = \"{}\"\nargs = \"{}\"", BINARY, subset.join(" ")))
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
            .trim_end_matches('\"');

        let file_path = dump_path.join(format!("test_{}.txt", args));
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }
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
