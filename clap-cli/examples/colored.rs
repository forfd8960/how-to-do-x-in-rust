use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!(
        "{} {}",
        "✓".green(),
        format!("Hello, {}!", args.name).bold()
    );
}
