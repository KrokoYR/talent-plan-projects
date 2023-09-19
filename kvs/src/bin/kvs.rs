use clap::{Parser, Subcommand};
use kvs::{KvError, KvStore};
use std::env::current_dir;

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

fn main() -> Result<(), KvError> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Get { key }) => {
            let mut kv_store = KvStore::open(current_dir()?)?;
            match kv_store.get(key.to_owned()) {
                Ok(maybe_key) => match maybe_key {
                    Some(v) => {
                        println!("{}", v);
                    }
                    None => {
                        println!("Key not found");
                    }
                },
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };
        }
        Some(Commands::Set { key, value }) => {
            let mut kv_store = KvStore::open(current_dir()?)?;
            kv_store.set(key.to_owned(), value.to_owned())?;
        }
        Some(Commands::Rm { key }) => {
            let mut kv_store = KvStore::open(current_dir()?)?;
            match kv_store.remove(key.to_owned()) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };
        }
        None => unreachable!(),
    }

    Ok(())
}
