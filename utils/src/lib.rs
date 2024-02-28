//! # utils
//!
//! A utils library for Layla-Chat.

// export the mods
pub mod init_block;
pub mod time;
pub mod exception;

/// # Initialization function
///
/// Initialize the necessary stuff for the project.
/// This is the only place where load the .env file.
pub fn init() {
    dotenv::dotenv().ok();
}

/// Define a trait to allow the struct can be stored or able to logged or sent to network.
/// 
/// # Example
/// ```
/// struct Foo {
///      // Something
/// }
///
/// impl Serializable for Foo {
///     fn to_string -> String {
///         // some codes can return a detailed string.
///     }
/// 
///     fn from_string() -> ErrorStruct<Self> where Self:  Sized {
///         // some codes can return a new struct For through the given string.
///         // the returned For should include all the details as the one we use to generate the string.
///         // sometimes, there will be some problems like wrong format or unmatched arguments, this will cause the failure in converting.
///         // when facing these problems, an error should be given out.
///     }
/// }
/// ```
///
/// # Error
/// When sometimes due to some unexpected issues, we cannot successfully convert the string to struct, the ErrorStruct::Err will be given as the output value.
pub trait Serializable {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> exception::error::ErrorStruct<Self> where Self: Sized;
    fn to_bytes(&self) -> Vec<u8> {
        // a standard by not comprehensive realization.
        self.to_string().into_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> exception::error::ErrorStruct<Self> where Self: Sized {
        // a standard by not comprehensive realization.
        let s = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => return exception::error::ErrorStruct::default_err(exception::error::ErrorType::FailedToParse),
        };
        match Self::from_string(s) {
            exception::error::ErrorStruct::Ok(obj) => exception::error::ErrorStruct::Ok(obj),
            exception::error::ErrorStruct::Err(e) => exception::error::ErrorStruct::Err(e),
        }
    }
}