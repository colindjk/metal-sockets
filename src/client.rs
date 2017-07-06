use mio::*;
use mio::tcp::{TcpStream};

use std::net::SocketAddr;
use std::io::{Read, Write, Result};

// Here, we redefine the read function for the Client. This way we can use
// the read function as a point of entry for HTTP parsing code.
pub struct Client {
    stream: TcpStream,
    addr: String,

}

impl Client {

    pub fn new(stream: TcpStream, addr: String) -> Self {
        Client { stream, addr }
    }
}

impl Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}

