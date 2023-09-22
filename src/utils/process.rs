use std::io::{self};
use std::process::{Command, Output};

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
