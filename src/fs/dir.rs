use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Dumps a set of strings into individual files within a specified directory.
///
/// This function creates a directory (if it doesn't exist) and writes each
/// string from the provided `output_strings` vector to a new file within that
/// directory. The filename is derived from the content of the string,
/// specifically from the second line which is expected to contain an "args"
/// field.
///
/// # Parameters
///
/// - `dump_dir`: The path to the directory where the files should be dumped.
/// - `output_strings`: A vector of strings where each string represents the content to be written
///   to a new file in the dump directory.
///
/// # Panics
///
/// This function will panic if:
/// - It fails to create the dump directory.
/// - It fails to create a file within the dump directory.
/// - It fails to write to a file.
///
/// # Examples
///
/// ```rust
/// use dir::dump_dir;
///
/// let contents = vec![
///     String::from("Some content with\nargs = \"tests/itest\"\nMore content"),
///     String::from("Another content with\nargs = \"sample_arg\"\nEven more content"),
/// ];
///
/// dump_dir("./dumps", contents);
/// ```
pub fn dump_dir(dump_dir: &str, output_strings: Vec<String>) {
    // Create the dump directory if it doesn't exist
    let dump_path = Path::new(dump_dir);
    if !dump_path.exists() {
        std::fs::create_dir(dump_path)
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
}
