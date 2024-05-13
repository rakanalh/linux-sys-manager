use std::process::{Command, ExitStatus};

pub fn execute_command(command: String) -> anyhow::Result<ExitStatus> {
    let mut parts = command.split(" ").collect::<Vec<&str>>();
    let binary = parts.remove(0);
    let mut command = Command::new(binary);
    let command = command.args(&parts);
    let output = command.output()?;
    Ok(output.status)
}
