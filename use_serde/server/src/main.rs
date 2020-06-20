use serde::{Deserialize, Serialize};
use serde_json;

use std::io::{self, prelude::*, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

#[derive(Debug, Deserialize, Serialize)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(|| {
                    handle_client(s).unwrap_or_else(|e| eprintln!("error: {}", e));
                });
            }
            Err(e) => eprint!("error: {}", e),
        }
    }
    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    println!("Incoming from: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut buf_reader = BufReader::new(stream);

    loop {
        data.clear();
        let bytes_read = buf_reader.read_until(b'\n', &mut data)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let input: Point3D = serde_json::from_slice(&data)?;
        println!("Received: {:?}", input);
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
        let res = serde_json::to_vec(&((value as f64).sqrt()))?;

        buf_reader.get_mut().write(&res)?;
        buf_reader.get_mut().write(b"\n")?;
        buf_reader.get_mut().flush()?;
    }
}
