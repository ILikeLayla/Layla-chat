use tokio::net::{TcpSocket, TcpStream};
use tokio::io::BufWriter;
use bytes::BytesMut;
use std::env;


pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let size = env::var("BUFFER_SIZE").unwrap_or("4096".to_string());
        let size = size.parse::<usize>().unwrap_or(4096);
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(size),
        }
    }
}