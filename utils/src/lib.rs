//! # utils
//!
//! An utils library for Layla-Chat.

// export the mods
pub mod init_block;
pub mod time;
pub mod exception;
pub mod en_de;
pub mod setting;

/// # Initialization function
///
/// Initialize the necessary stuff in utils for the project.
/// This is the only place where loads the .env file.
pub fn init() {
    dotenv::dotenv().unwrap();
    unsafe {
        GLOBAL_SETTING.init();
    }
}

// Load the global setting table.
pub static mut GLOBAL_SETTING: init_block::InitBlock<setting::Setting> = init_block::InitBlock { item: None };

/// Define a trait to allow the struct can be stored or able to logged or sent to network.
/// 
/// # Example
/// ```
/// struct Foo {
///      // Something
/// }
///
/// impl Serializable for Foo {
///     fn to_string_serial() -> String { /* something to generate a detailed string. */ }
/// 
///     fn from_str(s: &str) -> ErrorStruct<Self> where Self:  Sized {
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
    fn to_string_serial(&self) -> String;
    fn from_str(s: &str) -> exception::ErrorStruct<Self> where Self: Sized;
    fn to_bytes(&self) -> Vec<u8> {
        // a standard by not comprehensive realization.
        self.to_string_serial().into_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> exception::ErrorStruct<Self> where Self: Sized {
        // a standard by not comprehensive realization.
        let s = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => return exception::ErrorStruct::default_err(exception::ErrorType::FailedToParse),
        };
        match Self::from_str(s) {
            exception::ErrorStruct::Ok(obj) => exception::ErrorStruct::Ok(obj),
            exception::ErrorStruct::Err(e) => exception::ErrorStruct::Err(e),
        }
    }
}

impl Serializable for String {
    fn to_string_serial(&self) -> String {
        self.clone()
    }

    fn from_str(s: &str) -> exception::ErrorStruct<Self> where Self: Sized {
        exception::ErrorStruct::Ok(s.to_string())
    }
}

/// Check if s is with standard format. It will return the `Data` by `ErrorStruct::OK`.
/// 
/// # STANDARD FORMAT 
/// Label<...Data...>
/// 
/// # Error
/// This function will return `WrongFormat` when the s is not with the standard format.
pub fn format_check<'a>(s: &'a str, label: &str) -> exception::ErrorStruct<&'a str> {
    let label_length = label.len();
    if s.len() > label_length + 2 {
        if &s[..label_length] == label && &s[label_length..(label_length+1)] == "<" && &s[(s.len()-1)..s.len()] == ">"{
            return exception::ErrorStruct::Ok(&s[label_length+1..(s.len()-1)])
        } else {
            return exception::ErrorStruct::default_err(exception::ErrorType::WrongFormat)
        }
    } else {
        exception::ErrorStruct::default_err(exception::ErrorType::WrongFormat)
    }
}