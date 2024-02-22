use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref STANDARD_ERROR_DESCRIPTION: HashMap<usize, &'static str> = HashMap::from([
        (0, "Cannot find the id."),
        (1, "The id is already occupied."),
        (2, "Need to initialize the object."),
        // (3, "Wrong format is given.")
    ]);
}

pub mod error {
    use super::STANDARD_ERROR_DESCRIPTION;

    #[derive(Clone, Debug, Copy)]
    pub enum ErrorType {
        IdNotFound = 0,
        IdOccupied = 1,
        NeedToInit = 2,
        // WrongFormat = 3,
    }

    #[derive(Debug, Clone)]
    pub struct Error {
        error: ErrorType,
        description: String,
    }

    impl Error {
        pub fn new(error: ErrorType, description: String) -> Self {
            Self { error, description }
        }

        pub fn default(error: ErrorType) -> Self {
            let error_id = error.clone() as usize;
            let description = STANDARD_ERROR_DESCRIPTION.get(&error_id).unwrap();
            Self::new(error, description.to_string())
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error<error<{:?}>,description<{}>>", self.error, self.description)
        }
    }

    impl std::error::Error for Error {}

    pub enum ErrorStruct<T> {
        Ok(T),
        Err(Error)
    }

    // impl<T: From<String>> From<String> for ErrorStruct<T> {
    //     fn from(value: String) -> Self {
    //         ErrorStruct::Ok(T::from(value))
    //     }
    // }
}