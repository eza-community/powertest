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
