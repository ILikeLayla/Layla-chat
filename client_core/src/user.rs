// use super::format_checker;
// use std::hash::{Hasher, Hash};
use core_base::RawUser;
use core_base::context::Context;

#[derive(PartialEq, Eq, Clone)]
pub struct User {
    raw: RawUser,
    icon: Context,
}

impl User {
    pub fn get_id(&self) -> String {
        self.raw.get_id()
    }
}