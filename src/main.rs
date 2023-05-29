use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Drone plugin for Rust", long_about = None)]
struct Args {
    /// Pattern to look for
    #[arg(short, long, env = "PLUGIN_PATTERN")]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.pattern);
    println!("Answer {}", drone_rust::answer());
}
