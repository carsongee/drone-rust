use std::io::{self, Write};
use std::process::Command;

pub fn run_command(command: &str, dry_run: bool, mut writer: impl std::io::Write) {
    writeln!(writer, "{}", format!("> {command}")).unwrap();
    if dry_run {
        return;
    };
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Command failed to execute");
    writer.write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_command_dry_run() {
        let mut result = Vec::new();
        run_command("echo hello", true, &mut result);
        let output = String::from_utf8_lossy(&result);
        assert!(!output.contains("\nhello\n"));
    }

    #[test]
    fn test_run_command() {
        let mut result = Vec::new();
        run_command("echo hello", false, &mut result);
        let output = String::from_utf8_lossy(&result);
        println!("{}", output);
        assert!(output.contains("\nhello\n"));
    }
}
