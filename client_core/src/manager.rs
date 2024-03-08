use super::{User, Group};
use std::collections::HashMap;
use utils::exception::*;

pub struct Manager {
    group_list: HashMap<String, Group>,
    user_list: HashMap<String, User>
}

impl Manager {
    pub fn new<'a>() -> Manager {
        Self { 
            group_list: HashMap::new(),
            user_list: HashMap::new()
        }
    }

    pub fn get_user(&self, id: &str) -> ErrorStruct<&User> {
        return match self.user_list.get(id) {
            Some(user) => ErrorStruct::Ok(user),
            None => ErrorStruct::Err(Error::default(ErrorType::IdNotFound))
        }
    }

    pub fn add_user(&mut self, user: User) -> ErrorStruct<&User> {
        let id_buffer = user.get_id();
        return if self.check_user_exist(&id_buffer) {
            ErrorStruct::Err(Error::default(ErrorType::IdOccupied))
        } else {
            let _ = self.user_list.insert(id_buffer.clone(), user);
            self.get_user(&id_buffer)
        }
    }

    pub fn check_user_exist(&self, id: &str) -> bool {
        return self.user_list.contains_key(id);
    }
}

impl utils::init_block::InitAble for Manager {
    fn init() -> Self {
        Self::new()
    }
}