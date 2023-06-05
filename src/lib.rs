use std::env::set_var;
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

pub fn set_environment(env_vars: &json::JsonValue) {
    if !env_vars.is_object() {
        println!("`env` must be a JSON object of str:str");
        return;
    }
    for (key, value) in env_vars.entries() {
        println!("Setting environment variable: {}: {}", key, value);
        set_var(key, value.as_str().unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::{remove_var, var};

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

    #[test]
    fn test_set_environment() {
        assert!(var("HI").is_err());
        set_environment(&json::parse("123").unwrap());
        assert!(var("HI").is_err());
        set_environment(&json::parse("[1,2,3]").unwrap());
        assert!(var("HI").is_err());
        set_environment(&json::parse(r#"{"HI": "Hello"}"#).unwrap());
        assert!(var("HI").unwrap() == "Hello");
        remove_var("HI");
    }
}
