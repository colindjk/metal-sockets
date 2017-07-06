// Take a really long time with something simple, and it gets complicated.

extern crate mio;
extern crate http_muncher;

use mio::*;
use mio::tcp::{TcpListener};

use std::io::{Read, Write};
use std::collections::HashMap;

use std::str::from_utf8;

mod client;
mod server;

const SERVER: Token = Token(0);

fn main() {
    let addr = "127.0.0.1:8000".parse().unwrap();
    let server = TcpListener::bind(&addr).unwrap();

    let poll = Poll::new().unwrap();

    // poll.register(handle: &E, token: Token, interest: Ready, opts: PollOpt)
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

    // Here is where we "store" events.
    let mut events = Events::with_capacity(1024);
    let mut buf : [u8; 2048] = [0; 2048];
    let mut streams = HashMap::new();

    let mut client_no : usize = 1;

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                SERVER => {
                    let (stream, _addr) = server.accept().unwrap();
                    let token = Token::from(client_no);
                    client_no += 1;

                    if let Some(_stream) = streams.insert(token, stream) {
                        panic!("Stream entry token filled.");
                    }

                    poll.register(&streams[&token], token, Ready::readable(),
                                  PollOpt::edge()).unwrap();
                }
                client_token => {
                    let stream = streams.get_mut(&client_token).unwrap();
                    stream.read(&mut buf).unwrap();
                    stream.write(&buf);
                    println!("Received Bytes: {}", from_utf8(&buf).unwrap());
                }
            }

            if event.token() == SERVER && event.readiness().is_writable() {
                println!("exitings... idk why");
                return;
            }
        }
    }

}

// streams[&mut thing] crashed the compiler
