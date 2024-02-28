// use crate::exception::ErrorStruct;
use super::{User, group::Group};
use std::time::SystemTime;
use std::sync::Arc;
use utils::time::Time;

#[derive(Clone)]
pub struct Message {
    group_id: String,
    from: Arc<User>,
    time: Time,
    ctx: Context,
}

#[derive(Clone)]
pub struct MsgStream {
    group_id: String,
    from: Arc<User>,
    time: Time,
    ctx: CtxStream,
}

#[derive(Clone)]
pub struct Context;

impl Context {
    pub fn new() -> Self {
        Context {}
    }
}

#[derive(Clone)]
pub struct CtxStream;

// impl From<String> for ErrorStruct<Context> {
//     fn from(value: String) -> Self {
//         let error_buffer = format_checker(value, "Ctx<", ">");

//         return match error_buffer {
//             ErrorStruct::Ok(_) => ErrorStruct::Ok(Context::new()),
//             ErrorStruct::Err(err) => ErrorStruct::Err(err),
//         }
//     }
// }

impl From<Context> for String {
    fn from(_value: Context) -> Self {
        String::from("Ctx<>")
    }
}

// impl Translate for SystemTime {
//     fn to_string(&self) -> String {
//         format!("SystemTime<>")
//     }

//     fn from_string(s: String) -> Result<Self, crate::exception::Error> where Self: Sized {
//         let _ = format_checker(s, "SystemTime<", ">")?;
//         Ok(SystemTime::now())
//     }
// }