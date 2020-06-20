use serde::{Deserialize, Serialize};
use serde_json;

use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

#[derive(Debug, Deserialize, Serialize)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let components: Vec<&str> = input.trim_matches('\n').split(',').collect();
        if components.len() != 3 {
            eprintln!("Invalid input");
            panic!();
        }
        println!("Components: {:?}", components);
        let point = Point3D {
            x: components[0].trim().parse().unwrap_or_default(),
            y: components[1].trim().parse().unwrap_or_default(),
            z: components[2].trim().parse().unwrap_or_default(),
        };

        stream.write(serde_json::to_string(&point).unwrap_or_default().as_bytes())?;
        stream.write(b"\n")?;

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer)?;
        let input = str::from_utf8(&buffer).unwrap();
        if input.is_empty() {
            eprintln!("Empty res");
        }
        println!("res: {}", input);
    }
}
