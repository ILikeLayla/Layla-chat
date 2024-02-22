use crate::exception::ErrorStruct;
use super::{User, group::Group, format_checker};
use std::time::SystemTime;
use std::sync::Arc;

#[derive(Clone)]
pub struct Message {
    group_id: String,
    from: Arc<User>,
    time: SystemTime,
    ctx: Context,
}

#[derive(Clone)]
pub struct Context;

impl Context {
    pub fn new() -> Self {
        Context {}
    }
}

impl From<String> for ErrorStruct<Context> {
    fn from(value: String) -> Self {
        let error_buffer = format_checker(value, "Ctx<", ">");

        return match error_buffer {
            ErrorStruct::Ok(_) => ErrorStruct::Ok(Context::new()),
            ErrorStruct::Err(err) => ErrorStruct::Err(err),
        }
    }
}

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