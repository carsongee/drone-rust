use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Drone plugin for Rust", long_about = None)]
struct Args {
    /// List of commands (comma separated)
    #[arg(
        short,
        long,
        env = "PLUGIN_COMMANDS",
        value_delimiter = ',',
        required = false
    )]
    commands: Vec<String>,

    /// Key values of environment variables to set
    #[arg(short, long, env = "PLUGIN_ENV", required = false)]
    env: Option<json::JsonValue>,

    /// If dry_run is set/true print commands and actions
    #[arg(short, long, env = "PLUGIN_DRY_RUN", required = false)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    for command in &args.commands {
        drone_rust::run_command(command, args.dry_run, &mut std::io::stdout());
    }

    if let Some(env) = args.env {
        println!("{}", env);
    }
}
