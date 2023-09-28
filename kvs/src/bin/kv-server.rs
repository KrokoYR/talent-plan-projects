use clap::{Parser, ValueEnum};
use kvs::KvError;
use slog::*;

use std::env::current_dir;
use std::fmt;
use std::fs;
use std::net::SocketAddr;
use std::process::exit;
use std::str::FromStr;

use kvs::{KvEngine, KvServer, KvStore, Result, SledKvEngine};

#[derive(Parser, Debug)]
#[command(name = "ksv-server")]
#[command(version=env!("CARGO_PKG_VERSION"))]
#[command(author=env!("CARGO_PKG_AUTHORS"))]
#[command(about=env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[arg(long, value_name = "IP:PORT", default_value_t = String::from("127.0.0.1:4000"))]
    addr: String,

    #[arg(long, value_enum, default_value_t = Engine::Kvs)]
    engine: Engine,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Engine {
    /// Default(built-in) engine
    Kvs,

    /// Sled engine
    Sled,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kvs => {
                write!(f, "kvs")
            }
            Self::Sled => {
                write!(f, "sled")
            }
        }
    }
}

impl FromStr for Engine {
    type Err = KvError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "kvs" => Ok(Self::Kvs),
            "sled" => Ok(Self::Sled),
            _ => Ok(Self::Kvs),
        }
    }
}

fn main() -> Result<()> {
    let decorator = slog_term::PlainDecorator::new(std::io::stderr());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!("version" => "0.5"));

    let cli = Cli::parse();
    let ip_port = SocketAddr::from_str(&cli.addr)?;

    let curr_engine = current_engine()?;
    let engine = match curr_engine {
        Some(e) => {
            if e != cli.engine {
                error!(logger, "Wrong engine");
                exit(1);
            }

            e
        }
        None => {
            fs::write(current_dir()?.join("engine"), format!("{}", cli.engine))?;
            cli.engine
        }
    };

    info!(logger, "kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!(logger, "Storage engine: {}", engine);
    info!(logger, "Listening on {}", cli.addr);

    match engine {
        Engine::Kvs => {
            let kv_engine = KvStore::open(current_dir()?)?;
            run_server(kv_engine, logger, ip_port)?
        }
        Engine::Sled => {
            let sled_engine = SledKvEngine::new(sled::open(current_dir()?)?);
            run_server(sled_engine, logger, ip_port)?
        }
    };

    Ok(())
}

fn run_server<E: KvEngine>(engine: E, logger: slog::Logger, addr: SocketAddr) -> Result<()> {
    let mut server = KvServer::new(engine, logger)?;
    server.run(addr)
}

fn current_engine() -> Result<Option<Engine>> {
    let engine = current_dir()?.join("engine");
    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine)?.parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            println!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }
}
