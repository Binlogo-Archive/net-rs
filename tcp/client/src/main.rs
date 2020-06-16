use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        tcp_stream.write(input.as_bytes())?;

        let mut buf_reader = BufReader::new(&tcp_stream);
        let mut buf: Vec<u8> = Vec::new();
        buf_reader.read_until(b'\n', &mut buf).expect("failed");

        let string = str::from_utf8(&buf).unwrap();
        println!("{}", string);
    }
}
