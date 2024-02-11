use std::net::TcpStream;
use dotenv::dotenv;
use std::env;
use std::io::{Read, Write};
use std::str;

fn main() {
    dotenv().ok();
    let server_addr = env::var("SERVER_ADDR").unwrap();
    let server_port = env::var("SERVER_PORT").unwrap();

    let mut stream = TcpStream::connect(server_addr + ":" + &server_port).unwrap();
    stream.write("Layla".as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Response from server: {:?}", str::from_utf8(&buffer).unwrap())
    // println!("Hello, world!");
}
