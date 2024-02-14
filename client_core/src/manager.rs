use crate::{exception::{Error, ErrorStruct, Warn, WarnType}, User};
use super::group::Group;
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

    pub fn new_remote_user(&mut self, s: String) -> ErrorStruct<()> {
        let user: ErrorStruct<User> = s.into();
        let mut user = match user {
            ErrorStruct::Ok(u) => u,
            ErrorStruct::Err(e) => return ErrorStruct::Err(e)
        };
        let _ = self.user_list.insert(user.get_id(), user);
        ErrorStruct::Ok(())
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

