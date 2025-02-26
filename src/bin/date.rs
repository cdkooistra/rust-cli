use chrono::Utc;
use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", about = "Formats the current date/time")]
struct Args {
    // date format string
    format: String,
}

fn main() {
    let args = Args::parse();
    let now = Utc::now();

    println!("{}", now.format(&args.format));
}