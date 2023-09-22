use std::io::{self};
use std::process::{Command, Output};

/// Executes a given command with the `--help` argument and returns its output.
///
/// This function will run the provided command with the `--help` argument and capture its output.
/// If the command fails to execute successfully, an error will be printed to the standard error
/// and an `io::Error` will be returned.
///
/// # Parameters
///
/// - `command`: The name of the command to be executed.
/// - `_args`: Additional arguments for the command. (Currently unused in the function)
///
/// # Returns
///
/// Returns a `Result` containing the standard output of the command as a `Vec<u8>` on success,
/// or an `io::Error` on failure.
///
/// # Examples
///
/// ```rust
/// use utils::get_help;
///
/// let output = get_help("ls", &[]).unwrap();
/// println!("{}", String::from_utf8_lossy(&output));
/// ```
pub fn get_help(command: &str, _args: &[&str]) -> Result<Vec<u8>, std::io::Error> {
    let output: Output = Command::new(command).args(["--help"]).output()?;

    if !output.status.success() {
        eprintln!("Command failed with status: {:?}", output.status);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Command execution failed",
        ));
    }

    Ok(output.stdout)
}
