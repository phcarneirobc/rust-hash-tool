use clap::Parser;
use rust_hash_tool::Algorithm;

#[derive(Parser, Debug)]
#[command(name = "rust-hash-tool")]
#[command(about = "Calculate SHA256 or SHA512 hashes", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "sha256")]
    algorithm: Algorithm,
}

fn main() {
    let args = Args::parse();

    println!("Algorithm: {}", args.algorithm.display_name());
    println!("Input: {}", args.input);

    let hash = args.algorithm.hash(&args.input);
    println!("Hash: {}", hash);
}
