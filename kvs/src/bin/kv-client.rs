use clap::{Parser, Subcommand};
use kvs::{KvClient, KvError};
use std::net::SocketAddr;
use std::str::FromStr;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(Parser, Debug)]
#[command(name=env!("CARGO_PKG_NAME"))]
#[command(version=env!("CARGO_PKG_VERSION"))]
#[command(author=env!("CARGO_PKG_AUTHORS"))]
#[command(about=env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        /// The key to get
        key: String,
        ///
        #[arg(long, value_name=ADDRESS_FORMAT, default_value_t=DEFAULT_LISTENING_ADDRESS.to_string())]
        addr: String,
    },

    Set {
        /// The key to set
        key: String,
        /// The value to set
        value: String,
        ///
        #[arg(long, value_name=ADDRESS_FORMAT, default_value_t=DEFAULT_LISTENING_ADDRESS.to_string())]
        addr: String,
    },

    Rm {
        /// The key to remove
        key: String,
        ///
        #[arg(long, value_name=ADDRESS_FORMAT, default_value_t=DEFAULT_LISTENING_ADDRESS.to_string())]
        addr: String,
    },
}

fn main() -> Result<(), KvError> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Get { key, addr }) => {
            let ip_port = SocketAddr::from_str(addr.as_str())?;
            let mut client = KvClient::connect(ip_port)?;
            if let Some(value) = client.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Some(Commands::Set { key, value, addr }) => {
            let ip_port = SocketAddr::from_str(addr.as_str())?;
            let mut client = KvClient::connect(ip_port)?;
            client.set(key, value)?;
        }
        Some(Commands::Rm { key, addr }) => {
            let ip_port = SocketAddr::from_str(addr.as_str())?;
            let mut client = KvClient::connect(ip_port)?;
            client.remove(key)?;
        }
        None => unreachable!(),
    }

    Ok(())
}
