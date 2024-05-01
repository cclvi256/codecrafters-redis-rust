// Uncomment this block to pass the first stage
use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // Command line arguments
    let args: Vec<String> = env::args().collect();

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                
                loop {
                    let read_count = stream.read(&mut buffer).unwrap();
                    if read_count == 0 {
                        break;
                    }
                    
                    stream.write(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
