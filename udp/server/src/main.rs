use std::io::{self};
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    loop {
        let mut buf = [0; 512];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let filled_buf = &mut buf[..amt];
        socket.send_to(&filled_buf, src)?;
    }
}
