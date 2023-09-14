use regex::Regex;
use std::io::{self, BufRead};

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
    // println!("Set: {:?}", set);

    let powerset = generate_powerset(&set, DEPTH);
    // println!("Powerset: {:?}", powerset);

    let output_strings: Vec<String> = powerset
        .iter()
        .map(|subset| format!("bin.name = \"{}\"\nargs = \"{}\"", BINARY, subset.join(" ")))
        .collect();

    println!("Output Strings: {:#?}", output_strings);
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
