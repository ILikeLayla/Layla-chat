use std::collections::HashMap;
use std::sync::Arc;
use super::RawUser;
use super::message::Message;
use utils::exception::{ErrorStruct, ErrorType};

pub struct RawGroup {
    id: String,
    password: String,
    name: String,
    user_list: HashMap<String, Arc<RawUser>>,
    msg_list: Vec<Message>,
    admin: Arc<RawUser>,
}

impl RawGroup {
    pub fn add_user(&mut self, user: Arc<RawUser>) {
        let _ = self.user_list.insert(user.as_ref().get_id(), user);
    }

    pub fn get_msg_iter(&self) -> impl std::iter::Iterator<Item = &Message> + '_ { self.msg_list.iter() }
    pub fn get_user_iter(&self) -> impl std::iter::Iterator<Item = &Arc<RawUser>> + '_ { self.user_list.values() }

    pub fn check_password(&self, password: &str) -> bool { password == self.password }
    pub fn change_password(&mut self, current: &str, new: &str) -> ErrorStruct<()> {
        if self.check_password(current) {
            self.password = new.to_string();
            ErrorStruct::Ok(())
        } else {
            ErrorStruct::error(ErrorType::WrongArgument, "The password is wrong!.")
        }
    }
}