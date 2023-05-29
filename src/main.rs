use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about = "Drone plugin for Rust", long_about = None)]
struct Args {
    /// Pattern to look for
    #[arg(short, long, env = "PLUGIN_COMMANDS", value_delimiter = ',')]
    commands: Vec<String>,
}

fn run_command(command: &String, dry_run: bool) {
    println!("{}", format!("> {command}"));
    if dry_run {
        println!("Dry run");
    }
}

fn main() {
    let args = Args::parse();
    for command in &args.commands {
        run_command(command, true);
    }
    println!("Answer {}", drone_rust::answer());
}
