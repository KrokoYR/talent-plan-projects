use crate::KvError;
use std::io::{BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::result;

use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};

type Result<T = ()> = result::Result<T, KvError>;

/// Client implementation
pub struct KvClient {
    reader: TcpStream,
    writer: TcpStream,
}

impl KvClient {
    ///
    pub fn connect(ip_port: SocketAddr) -> Result<Self> {
        let tcp_reader = TcpStream::connect(ip_port)?;
        let tcp_writer = tcp_reader.try_clone()?;

        Ok(Self {
            reader: tcp_reader,
            writer: tcp_writer,
        })
    }

    ///
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let request = Request::Get { key };
        let bson_request = bson::to_vec(&request)?;
        self.writer.write(&bson_request)?;

        let reader = BufReader::new(&self.reader);
        let resp: GetResponse = bson::from_reader(reader)?;

        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(msg) => Err(KvError::StringError(msg)),
        }
    }

    ///
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let request = Request::Set { key, value };
        let bson_request = bson::to_vec(&request)?;
        self.writer.write(&bson_request)?;

        let reader = BufReader::new(&self.reader);
        let resp: SetResponse = bson::from_reader(reader)?;

        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(msg) => Err(KvError::StringError(msg)),
        }
    }

    ///
    pub fn remove(&mut self, key: String) -> Result<()> {
        let request = Request::Remove { key };
        let bson_request = bson::to_vec(&request)?;
        self.writer.write(&bson_request)?;

        let reader = BufReader::new(&self.reader);
        let resp: RemoveResponse = bson::from_reader(reader)?;

        match resp {
            RemoveResponse::Ok(_) => Ok(()),
            RemoveResponse::Err(msg) => Err(KvError::StringError(msg)),
        }
    }
}
