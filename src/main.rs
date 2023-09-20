#![allow(unused)]

use clap::{Args, Parser, Subcommand};
use crate::util::hex;

pub mod util;

enum HexTool {
    Hex,
    UnHex,
}

impl HexTool {
    fn process(&self, input: &str, numeric: bool, split: bool) -> String {
        match self {
            HexTool::Hex => hex::convert(input, numeric, split),
            HexTool::UnHex => input.as_bytes().iter().map(|b| format!("{:02x}", b)).collect(),
        }
    }
}


#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
/// Commandline tool to convert hex to string and string to hex
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Change input to hex
    Hex(Hex),
    /// Change input from hex
    Unhex(Unhex),
}

#[derive(Args)]
struct Hex {
    #[clap(short, long, action)]
    /// Numeric flag, if set, will assume input is a number and convert to hex
    numeric: bool,

    #[clap(short, long, action)]
    /// Split flag, if set, will split the output into bytes
    split: bool,

    /// Input to convert
    input: String,
}

#[derive(Args)]
struct Unhex {
    #[clap(short, long, action)]
    /// Numeric flag, if set, will assume input is a number and convert to hex
    numeric: bool,

    #[clap(short, long, action)]
    /// Split flag, if set, will split the output into bytes
    split: bool,

    /// Input to convert
    input: String,
}


fn main() {
    let args = CLI::parse();
    let result = match &args.command {
        Commands::Hex(h) => HexTool::Hex.process(&h.input, h.numeric, h.split),
        Commands::Unhex(u) => HexTool::Hex.process(&u.input, u.numeric, u.split),
    };
    print!("{}", result);
}
