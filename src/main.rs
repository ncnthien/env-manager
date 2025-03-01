use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    parameter: String
}

fn main() {
    println!("Hello, world!");
}
