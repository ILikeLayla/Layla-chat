pub trait InitAble {
    fn init() -> Self;
}

pub struct InitBlock<T: InitAble> {
    item: Option<T>
}

impl<T: InitAble> InitBlock<T> {
    pub fn new() -> Self {
        Self {
            item: None
        }
    }

    pub fn init(&mut self) {
        self.item = Some(T::init())
    }

    pub fn status(&self) -> bool {
        self.item.is_some()
    }
}