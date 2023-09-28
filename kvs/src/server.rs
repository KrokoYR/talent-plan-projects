use slog::{error, info, Logger};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::result;

use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KvEngine, KvError};

///
pub struct KvServer<E: KvEngine> {
    engine: E,
    logger: Logger,
}
type Result<T = ()> = result::Result<T, KvError>;

impl<E: KvEngine> KvServer<E> {
    ///f
    pub fn new(engine: E, logger: Logger) -> Result<Self> {
        Ok(KvServer { engine, logger })
    }

    /// Run the server listening on the given address
    pub fn run<A: ToSocketAddrs>(&mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handle_connection(stream) {
                        error!(self.logger, "Error on serving client: {}", e);
                    }
                }
                Err(e) => error!(self.logger, "Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn handle_connection(&mut self, stream: TcpStream) -> Result<()> {
        let peer_addr = stream.peer_addr()?;
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let req: Request = bson::from_reader(reader)?;

        macro_rules! send_resp {
            ($resp:expr) => {{
                let resp = $resp;
                let buf = bson::to_vec(&resp)?;
                writer.write(&buf)?;
                writer.flush()?;
                info!(self.logger, "Response sent to {}: {:?}", peer_addr, resp);
            }};
        }

        match req {
            Request::Get { key } => send_resp!(match self.engine.get(key) {
                Ok(value) => GetResponse::Ok(value),
                Err(e) => GetResponse::Err(format!("{}", e)),
            }),
            Request::Set { key, value } => send_resp!(match self.engine.set(key, value) {
                Ok(_) => SetResponse::Ok(()),
                Err(e) => SetResponse::Err(format!("{}", e)),
            }),
            Request::Remove { key } => send_resp!(match self.engine.remove(key) {
                Ok(_) => RemoveResponse::Ok(()),
                Err(e) => RemoveResponse::Err(format!("{}", e)),
            }),
        };

        Ok(())
    }
}
