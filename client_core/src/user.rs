use super::format_checker;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use std::time::SystemTime;
use super::config::UNACCEPTABLE_NAME_CHAR;
use super::MANAGER;
// use logger::exception::error::{Error, ErrorType, ErrorStruct};
use logger::exception::error::*;

#[derive(PartialEq, Eq, Clone)]
pub struct User {
    name: String,
    id: String,
}

impl User {
    pub fn new(name: &str, id: &str) -> Result<Self, Error> {
        for i in UNACCEPTABLE_NAME_CHAR.iter() {
            if name.contains(i) {
                return Err(Error::default(ErrorType::WrongFormat))
            }
        };
        Ok(Self { name: name.to_string(), id: id.to_string()})
    }

    pub fn default(name: &str) -> Result<Self, Error> {
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

impl From<String> for &User {
    fn from(value: String) -> Self {
        let data = format_checker(value, "User<", ">");

        let data = match data {
            ErrorStruct::Ok(s) => s,
            ErrorStruct::Err(err) => return ErrorStruct::Err(err),
        };

        let label = ["name", "id"];

        let mut buffer = Vec::new();
        for (place, i) in data.split(",").enumerate() {
            let temp = format_checker(i.to_string(), &format!("{}<", label[place]), ">");
            buffer.push(temp)
        }

        let mut buffer1 = Vec::new();

        for i in buffer.iter() {
            buffer1.push(match &i {
                &ErrorStruct::Ok(s) => s,
                &ErrorStruct::Err(e) => return ErrorStruct::Err(e.clone())
            })
        }

        let user_check = MANAGER.check_user_exist(&buffer1[1]);
        return match user_check {
            true => {
                MANAGER.get_user(&buffer1[1])
            },
            false => unsafe {
                MANAGER.add_user(User { name: buffer1[0].clone(),  id: buffer1[1].clone() })
            }
        }
    }
}

impl From<User> for String {
    fn from(value: User) -> Self {
        format!("User<name<{}>,id<{}>>", value.name, value.id)
    }
}

impl From<&User> for String {
    fn from(value: &User) -> Self {
        format!("User<name<{}>,id<{}>>", value.name, value.id)
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_buffer: String = self.into();
        write!(f, "{}", display_buffer)
    }
}