mod message;
mod user;
mod manager;
mod request;
mod config;
mod group;
mod service;

pub use message::*;
pub use user::*;
pub use manager::*;
pub use request::*;
pub use service::*;

use lazy_static::lazy_static;
use dotenv::dotenv;
use logger::exception::error::*;
use utils::init_block::InitBlock;

static mut MANAGER: InitBlock<Manager> = InitBlock::new();

pub fn init() {
    dotenv().ok();
    unsafe {
        MANAGER.init();
    }
}

pub fn format_checker(s: String, start: &str, end: &str) -> String {
    if s.starts_with(start) && s.ends_with(end) {
        let start_place = start.len();
        let end_place = s.len() - end.len();
        let out_buf = &s[start_place..end_place];
        return out_buf.to_string()
    } else {
        // return ErrorStruct::Err(Error::default(ErrorType::WrongFormat))
        // panic!("Wrong format is given.")
    }
}