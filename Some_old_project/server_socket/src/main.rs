use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    connectionListener(listener);
}

fn connectionListener(listener: TcpListener) {
    let mut sockets: Vec<TcpStream> = Vec::new();

    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                //sockets.push(socket);
                println!("Socket connected on address: {:?}", addr);
                thread::spawn(move || {
                    socket.set_read_timeout(None).expect("Read timeout failed");
                    socket.set_nonblocking(false).expect("Non block failed");
                    socketListener(socket);
                });
            },
            Err(e) => println!("couldn't get client {:?}", e),
        }
    }
    
}

fn socketListener(mut socket: TcpStream) {
    loop {
        let mut buff = [0; 100];
        //let mut buffer = String::new();
        let result = socket.read(&mut buff);
        /*match result {
            Ok(res) => println!("buff filled with: {:?}", res),
            Err(e) => println!("Failed because {:?}", e),
        }*/
    }
}