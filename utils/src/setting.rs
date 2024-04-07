use std::collections::HashMap;
use super::exception::ErrorStruct;
use super::init_block::InitAble;

pub struct Setting<'a> {
    setting_set: HashMap<&'a str, &'a str>,
}

impl Setting<'_> {
    pub fn new() -> Self {
        Self {
            setting_set: HashMap::new(),
        }
    }
}

impl<'a> Setting<'a> {
    pub fn insert(&mut self, key: &'a str, value: &'a str) -> Option<&str> {
        self.setting_set.insert(key, value)
    }

    pub fn delete(&mut self, key: &str) {
        self.setting_set.remove(key);
    }

    pub fn get(&self, key: &str) -> ErrorStruct<&str> {
        ErrorStruct::from_option(self.setting_set.get(key)).ok_work(&|x| {**x})
    }

    pub fn get_mut(&'a mut self, key: &str) -> ErrorStruct<&mut &str> {
        ErrorStruct::from_option(self.setting_set.get_mut(key))
    }
}

impl InitAble for Setting<'_> {
    fn init() -> Self {
        Self::new()
    }
}