use super::User;

use std::time::SystemTime;

#[derive(Clone)]
pub enum RequestType {
    EnterRequest(EnterRequest),
    Other
}

#[derive(Clone)]
pub struct Request {
    request_type: RequestType,
    time: SystemTime,
    description: Option<String>,
}

impl Request {
    pub fn new(request_type: RequestType, description: Option<String>) -> Self {
        Self {
            request_type, description,
            time: SystemTime::now()
        }
    }
}

#[derive(Clone)]
pub struct EnterRequest {
    group_id: String,
    group_password: String,
    user: User,
}