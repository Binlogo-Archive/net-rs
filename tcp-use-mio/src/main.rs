use mio::net::{TcpListener, TcpStream};
use mio::*;
use std::error::Error;
use std::io::{self, Read, Write};

const SERVER_ACCEPT: Token = Token(0);
const SERVER: Token = Token(1);
const CLIENT: Token = Token(2);
const SERVER_HELLO: &[u8] = b"PING";
const CLIENT_HELLO: &[u8] = b"PONG";

fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:17442".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    let mut poll = Poll::new()?;
    poll.registry()
        .register(&mut server, SERVER_ACCEPT, Interest::READABLE)?;

    let mut client = TcpStream::connect(addr)?;
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    let mut events = Events::with_capacity(1024);

    let mut server_handler = None;

    loop {
        poll.poll(&mut events, None)?;

        for e in events.iter() {
            match e.token() {
                SERVER_ACCEPT => {
                    let (mut handler, addr) = server.accept()?;
                    println!("accept from addr: {}", &addr);
                    poll.registry().register(
                        &mut handler,
                        SERVER,
                        Interest::READABLE | Interest::WRITABLE,
                    )?;
                    server_handler = Some(handler);
                }
                SERVER => {
                    if e.is_writable() {
                        if let Some(ref mut handler) = &mut server_handler {
                            match handler.write(SERVER_HELLO) {
                                Ok(_) => println!("server pinged"),
                                Err(e) => eprintln!("server write err: {}", e),
                            }
                        }
                    }
                }
                CLIENT => {
                    if e.is_writable() {
                        match client.write(CLIENT_HELLO) {
                            Ok(_) => println!("client ponged"),
                            Err(e) => eprintln!("client write err: {}", e),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
