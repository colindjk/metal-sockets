use mio::*;
use mio::tcp::{TcpListener};
use std::net::SocketAddr;

use client::{Client};

use std::collections::HashMap;

pub struct Server {
    listener: TcpListener,
    clients: HashMap<Token, Client>,
    poll: Poll,
    token_counter: usize,
}

impl Server {

    // Creates a server and registers a listener on the poll.
    pub fn new(addr: &SocketAddr) -> Self {
        Server {
            listener: TcpListener::bind(addr).unwrap(),
            clients: HashMap::new(),
            poll: Poll::new().unwrap(),
            token_counter: 1,
        }
    }

}


