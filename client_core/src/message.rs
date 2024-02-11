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

// impl Translate for Message {
//     fn to_string(&self) -> String {
//         format!("Message<group_id<{}>,from<{}>,time<{}>,ctx<{}>>", 
//             self.group_id, Translate::to_string(self.from.as_ref()), self.time.to_string(), self.ctx.to_string()
//         )
//     }

//     fn from_string(s: String) -> Result<Self, crate::exception::Error> where Self: Sized {
//         let data = format_checker(s, "Message<", ">")?;

//         let mut buffer = Vec::new();
//         let separated_data: Vec<String> = data.split(",").map(|item| { item.to_string() }).collect();

//         let group_id = format_checker(separated_data[0], "group_id<", ">")?;

//     }
// }

#[derive(Clone)]
pub struct Context;

// impl Translate for Context {
//     fn to_string(&self) -> String {
//         format!("Ctx<>")
//     }

//     fn from_string(s: String) -> Result<Self, crate::exception::Error> where Self: Sized {
//         let _ = format_checker(s, "Ctx<", ">")?;
//         Ok(Self {})
//     }
// }

impl From<String> for Context {
    fn from(value: String) -> Self {
        
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