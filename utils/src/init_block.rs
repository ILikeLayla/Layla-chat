pub trait InitAble {
    fn init() -> Self;
    fn self_init(&mut self) where Self: Sized {
        *self = Self::init();
    }
}

impl<T: InitAble + Sized> InitAble for Option<T> {
    fn init() -> Self {
        Some(T::init())
    }
}

/// Allow to pre-announce a block in static that will be used globally.
/// The item in the block should be able to be initialized.
pub struct InitBlock<T: InitAble> {
    pub item: Option<T>
}

impl<T: InitAble> InitBlock<T> {
    pub fn init(&mut self) {
        self.item.self_init();
    }

    pub fn status(&self) -> bool {
        self.item.is_some()
    }

    pub fn get(&self) -> &T {
        &self.item.as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }
}