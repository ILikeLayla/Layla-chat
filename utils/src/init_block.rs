pub trait InitAble {
    ///  Initialization should not require any arguments.
    fn init() -> Self;

    /// replace the original one instead of giving another.
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
///
/// # Example
/// ```
/// struct Foo;
///
/// impl InitAble for Foo {
///     fn init() -> Self { /* Something to initialize the Foo */ }
/// }
/// 
/// static Bar: InitBlock<Foo> = InitBlock { item: None }
/// 
/// fn init() {
///     unsafe {
///         Bar.init{}
///     }
/// }
/// ```
pub struct InitBlock<T: InitAble> {
    /// Where the actual structure will be stored. The reason to be public is to allow to announce this block in static cell.
    pub item: Option<T>,
}

impl<T: InitAble> InitBlock<T> {
    /// Used to initialize the item filed that with "InitAble" trait.
    pub fn init(&mut self) {
        self.item.self_init();
    }

    /// Used to check the item had been initialized or not.
    pub fn status(&self) -> bool {
        self.item.is_some()
    }

    /// Get a reference to the item.
    pub fn get(&self) -> &T {
        &self.item.as_ref().unwrap()
    }

    /// Get a mutable reference to the item.
    pub fn get_mut(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }
}