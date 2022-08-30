use std::sync::mpsc::channel;
use std::thread;
use std::net::{TcpListener, TcpStream};

fn main() {

    let (sender, receiver) = channel();
    let mut children = Vec::new();
    
    for id in 0..10 {

        let threadSender = sender.clone();

        let child = thread::spawn(move || {
            match threadSender.send(id) {
                Ok(_v) => println!("{} sucessfully sent", id),
                Err(_e) => println!("{} failed to send", id),
            }
        });
        children.push(child);
    }

    let mut ids = Vec::with_capacity(10 as usize);
    for _ in 0..10 {
        ids.push(receiver.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("Error");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
    
}


fn listener() {
    let listener = TcpListener::bind("127.0.0.1:80")?;
}
