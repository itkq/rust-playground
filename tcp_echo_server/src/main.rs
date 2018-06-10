use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: &TcpStream) {
    let mut buf = [0; 256];

    let _ = stream.read(&mut buf[..]);
    let _ = stream.write(&buf);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Start TCP echo server.");

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client: {:?}", addr);

                thread::spawn(move || {
                    handle_client(&stream);
                });
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}
