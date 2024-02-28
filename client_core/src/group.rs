use std::collections::HashMap;
use std::sync::Arc;
use super::{User, Message};

pub struct Group {
    id: String,
    password: String,
    name: String,
    user_list: HashMap<String, Arc<User>>,
    msg_list: Vec<Message>
}

impl Group {
    pub fn add_user(&mut self, user: Arc<User>) {
        let _ = self.user_list.insert(user.as_ref().get_id(), user);
    }
}