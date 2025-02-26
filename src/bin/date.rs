use chrono::Utc;
use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", about = "Print datetime using formats")]
struct Args {
    #[arg(default_value = "%+")]
    format: String,
}

fn main() {
    let args = Args::parse();
    let now = Utc::now();

    println!("{}", now.format(&args.format).to_string());
}