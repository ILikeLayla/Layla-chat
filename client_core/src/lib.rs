mod net;
mod message;
mod user;
mod manager;
mod request;
// mod translator;
mod config;
mod group;
pub mod exception;

pub use message::*;
pub use net::*;
pub use user::*;
pub use manager::*;
pub use request::*;
// use translator::*;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MANAGER: Manager = Manager::new();
}

pub fn format_checker(s: String, start: &str, end: &str) -> exception::ErrorStruct<String>{
    if s.starts_with(start) && s.ends_with(end) {
        let start_place = start.len();
        let end_place = s.len() - end.len();
        let out_buf = &s[start_place..end_place];
        return exception::ErrorStruct::Ok(out_buf.to_string())
    } else {
        return exception::ErrorStruct::Err(exception::Error::default(exception::ErrorType::WrongFormat))
        // panic!("Wrong format is given.")
    }
}