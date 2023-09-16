use clap::{Parser, Subcommand};
use std::env;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(name=env!("CARGO_PKG_NAME"))]
#[command(version=env!("CARGO_PKG_VERSION"))]
#[command(author=env!("CARGO_PKG_AUTHORS"))]
#[command(about=env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    /// Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        /// The key to get
        key: String,
    },

    Set {
        /// The key to set
        key: String,
        /// The value to set
        value: String,
    },

    Rm {
        /// The key to remove
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Get { key: _ }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(Commands::Set { key: _, value: _ }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(Commands::Rm { key: _ }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        None => {
            unreachable!()
        }
    }
}

// Example for clap's "cargo" feature
// use std::path::PathBuf;

// use clap::{arg, command, value_parser};

// fn main() {
//     let matches = command!() // requires `cargo` feature
//         .arg(arg!([name] "Optional name to operate on"))
//         .arg(
//             arg!(
//                 -c --config <FILE> "Sets a custom config file"
//             )
//             // We don't have syntax yet for optional options, so manually calling `required`
//             .required(false)
//             .value_parser(value_parser!(PathBuf)),
//         )
//         .arg(arg!(
//             -d --debug ... "Turn debugging information on"
//         ));

//     println!("Matches: {:?}", matches);
// }
