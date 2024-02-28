use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref STANDARD_ERROR_DESCRIPTION: HashMap<usize, &'static str> = HashMap::from([
        (0, "Cannot find the id."),
        (1, "The id is already occupied."),
        (2, "Need to initialize the object."),
        (3, "Unacceptable argument is given."),
        (4, "Network issues."),
        (5, "Some unknown problems happened, nad no information was given for investigation."),
        (6, "Failed to initialize."),
        (7, "Failed to parse.")
    ]);
}

pub mod error {
    use super::STANDARD_ERROR_DESCRIPTION;

    #[derive(Clone, Debug, Copy)]
    pub enum ErrorType {
        IdNotFound = 0, 
        IdOccupied = 1,
        NeedToInit = 2,
        UnacceptableArgument = 3,
        NetWorkIssue = 4,
        UnknownProblem = 5,
        InitFailed = 6,
        FailedToParse = 7,
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

    impl<T> ErrorStruct<T> {
        pub fn default_err(error_type: ErrorType) -> Self {
            Self::Err(Error::default(error_type))
        }
    }
}
