#![allow(unused)]

use crate::cli::{Commands, HexTool};


fn main() {
    let args = cli::parse();
    let result = match &args.command {
        Commands::Hex { numeric, split, input } => {
            HexTool::Hex.process(input, *numeric, *split)
        }
        Commands::Unhex { numeric, split, input } => {
            HexTool::UnHex.process(input, *numeric, *split)
        }
    };
    print!("{}", result);
}


mod cli {
    use clap::{Args, Parser, Subcommand};
    use hextool::{Hex, UnHex, Convert};

    pub(crate) fn parse() -> CLI {
        CLI::parse()
    }

    pub(crate) enum HexTool {
        Hex,
        UnHex,
    }

    impl HexTool {
        pub(crate) fn process(&self, input: &str, numeric: bool, split: bool) -> String {
            match self {
                // HexTool::Hex(h) => h.convert(input, numeric, split),
                // HexTool::UnHex(u) => u.convert(input, numeric, split),
                HexTool::Hex => { Hex::convert(input, numeric, split) }
                HexTool::UnHex => { UnHex::convert(input, numeric, split) }
            }
        }
    }

    #[derive(Parser)]
    #[command(author, version, about)]
    #[command(propagate_version = true)]
    /// Commandline tool to convert hex to string and string to hex
    pub(crate) struct CLI {
        #[command(subcommand)]
        pub(crate) command: Commands,
    }

    #[derive(Subcommand)]
    pub(crate) enum Commands {
        /// Change input to hex
        Hex {
            #[clap(short, long, action)]
            /// Numeric flag, if set, will assume input is a number and convert to hex
            numeric: bool,

            #[clap(short, long, action)]
            /// Split flag, if set, will split the output into bytes
            split: bool,

            /// Input to convert
            input: String,
        },
        /// Change input from hex
        Unhex {
            #[clap(short, long, action)]
            /// Numeric flag, if set, will assume input is a number and convert to hex
            numeric: bool,

            #[clap(short, long, action)]
            /// Split flag, if set, will split the output into bytes
            split: bool,

            /// Input to convert
            input: String,
        },
    }
}
