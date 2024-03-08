// use super::format_checker;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use std::time::SystemTime;
// use logger::exception::error::{Error, ErrorType, ErrorStruct};
use utils::exception::*;

#[derive(PartialEq, Eq, Clone)]
pub struct RawUser {
    name: String,
    id: String,
}

impl RawUser {
    pub fn new(name: &str, id: &str) -> ErrorStruct<Self> {
        let char = std::env::var("UNACCEPTABLE_CHAR").unwrap_or(String::from(";<>."));
        for i in char.chars() {
            if name.contains(i) || id.contains(i) {
                return ErrorStruct::Err(Error::default(ErrorType::UnacceptableArgument))
            }
        };
        ErrorStruct::Ok(Self { name: name.to_string(), id: id.to_string()})
    }

    pub fn default(name: &str) -> ErrorStruct<Self> {
        let mut s = DefaultHasher::default();
        name.hash(&mut s);
        format!("{:?}", SystemTime::now()).hash(&mut s);
        let id = format!("{}", s.finish());
        Self::new(name, id.as_str())
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn change_id(&mut self, id: String) {
        self.id = id
    }
}


impl From<RawUser> for String {
    fn from(value: RawUser) -> Self {
        format!("User<name<{}>,id<{}>>", value.name, value.id)
    }
}

impl From<&RawUser> for String {
    fn from(value: &RawUser) -> Self {
        format!("User<name<{}>,id<{}>>", value.name, value.id)
    }
}

impl std::fmt::Display for RawUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_buffer: String = self.into();
        write!(f, "{}", display_buffer)
    }
}