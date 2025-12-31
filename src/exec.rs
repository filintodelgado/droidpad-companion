use std::{error, process::Command};

pub fn run_command(command: &String) -> Result<String, Box<dyn error::Error>> {
	let stdout = Command::new("sh").arg("-c").arg(command).output()?.stdout;

	let output = String::from_utf8(stdout).unwrap().trim().to_owned();

	Ok(output)
}
