#[allow(unused_imports)]
use std::env::args;

#[allow(unused_imports)]
use clap::{Parser, Subcommand};


/// Simple program to find and replace text in file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file to find text
    #[arg(short, long)]
    input: String,

    /// Name of the output file with replaced data
    #[arg(short, long)]
    output: String,

    /// Text that will be replaced
    #[arg(short, long)]
    find: String,

    /// New text that will replace the pattern
    #[arg(short, long)]
    replace: String,
}

use find_and_replace::replace;

fn main() {
    let args = Args::parse();
    let result = replace(&args.input, &args.find, &args.replace);
    println!("{}", result);
}
