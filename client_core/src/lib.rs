mod user;
mod manager;
mod service;
mod group;

pub use user::*;
pub use manager::*;
pub use service::*;
pub use group::*;

// use logger::exception::error::*;
use utils::init_block::InitBlock;

static mut MANAGER: InitBlock<Manager> = InitBlock{ item: None };

pub fn init() {
    utils::init();
    unsafe {
        MANAGER.init();
    }
}

// pub fn format_checker(s: String, start: &str, end: &str) -> String {
//     if s.starts_with(start) && s.ends_with(end) {
//         let start_place = start.len();
//         let end_place = s.len() - end.len();
//         let out_buf = &s[start_place..end_place];
//         return out_buf.to_string()
//     } else {
//         // return ErrorStruct::Err(Error::default(ErrorType::WrongFormat))
//         // panic!("Wrong format is given.")
//     }
// }