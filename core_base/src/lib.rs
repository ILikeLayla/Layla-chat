mod message;
pub mod request;
mod raw_user;
mod raw_group;
pub mod context;

pub use raw_user::*;
pub use raw_group::*;
pub use message::*;

#[derive(Clone)]
pub enum Frame {
    Message(Message),
    Request(request::Request),
}