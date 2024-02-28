use tokio::net::TcpSocket;
use super::{Request, Message};

pub enum Frame {
    Req(Request),
    Message(Message),
}