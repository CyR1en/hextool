use crate::cli::Commands;
use hextool::{Convert, Hex, UnHex};
use std::io;
use std::io::Write;

fn main() {
    let args = cli::parse();
    let result = match &args.command {
        Commands::Hex {
            numeric,
            split,
            input,
        } => Hex::convert(input, *numeric, *split),
        Commands::Unhex {
            numeric,
            split,
            input,
        } => UnHex::convert(input, *numeric, *split),
    };
    io::stdout().write_all(result.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

mod cli {
    use clap::{Parser, Subcommand};
    use std::ffi::OsString;
    use std::{env, io};

    pub(crate) fn parse() -> Cli {
        match Cli::try_parse() {
            Ok(c) => c,
            Err(_) => {
                // check if stdin is empty
                if atty::is(atty::Stream::Stdin) {
                    return Cli::parse();
                }
                parse_stdin()
            }
        }
    }

    fn parse_stdin() -> Cli {
        let mut input = String::from("");

        io::stdin().lines().for_each(|line| {
            input.push_str(&line.unwrap());
            input.push('\n');
        });

        let input = input.trim();
        let mut args: Vec<OsString> = env::args_os().collect();
        args.push(input.into());
        match Cli::try_parse_from(args) {
            Ok(c) => c,
            Err(e) => e.exit(),
        }
    }

    #[derive(Parser)]
    #[command(author, version, about)]
    #[command(propagate_version = true)]
    /// Commandline tool to convert hex to string and string to hex
    pub(crate) struct Cli {
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
