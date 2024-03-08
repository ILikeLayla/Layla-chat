// use crate::exception::ErrorStruct;
use super::RawUser;
use std::sync::Arc;
use utils::time::Time;
use super::context::Context;

#[derive(Clone)]
pub struct Message {
    group_id: String,
    from: Arc<RawUser>,
    time: Time,
    ctx: Context,
}