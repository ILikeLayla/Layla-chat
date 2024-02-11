use crate::{exception::{Error, Warn, WarnType}, User};
use super::{group::Group, config::ID_PLACEHOLDER};
use std::collections::HashMap;
use std::sync::Arc;

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

    pub fn new_user_by_string(&mut self, s: String) -> Result<(), Error>{
        let mut user: User = s.into();
        let mut current_id = user.get_id();
        while self.user_list.contains_key(&current_id) {
            current_id += ID_PLACEHOLDER
        };
        user.change_id(current_id);
        let _ = self.user_list.insert(current_id, user);
        Ok(())
    }

    pub fn add_user_to_group(&mut self, user_id:String, group_id:String) -> Result<(), Warn> {
        let user = match self.user_list.get(&user_id) {
            Some(buffer) => {
                Arc::new(*buffer)
            },
            None => return Err(Warn::default(WarnType::IdNotFound)),
        };
        let mut group = match self.group_list.get_mut(&group_id) {
            Some(a) => a,
            None => return Err(Warn::default(WarnType::IdNotFound))
        };
        group.add_user(user);
        Ok(())
    }
}

