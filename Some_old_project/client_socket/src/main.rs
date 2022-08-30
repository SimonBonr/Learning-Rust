use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3000")?;
    stream.write(b"some bytes")?;
    Ok(())
}
