use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = vec![];
    for stream in tcp_listener.incoming() {
        let stream = stream.expect("failed");
        let handle = thread::spawn(|| {
            handle_client(stream).unwrap_or_else(|e| eprintln!("{:?}", e));
        });
        thread_vec.push(handle);
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let str = String::from_utf8(buf.to_vec());
        println!("{:?}", str.unwrap());
        stream.write(&buf[..bytes_read])?;
    }
}
