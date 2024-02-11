use super::{Request, Message};

struct Queue<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn time_order(&self) -> Vec<T> {
        let mut buffer = self.data.clone();
        buffer.reverse();
        buffer
    }
}

pub struct Channel {
    request_queue: Queue<Request>,
    msg_queue: Queue<Message>
}