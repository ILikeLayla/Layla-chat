use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref STANDARD_ERROR_DESCRIPTION: HashMap<usize, &'static str> = HashMap::from([
        ( 0, "Cannot find the id."),
        ( 1, "The id is already occupied."),
        ( 2, "Need to initialize the object."),
        ( 3, "Unacceptable argument is given."),
        ( 4, "Network issues."),
        ( 5, "Some unknown problems happened, and no information was given for investigation."),
        ( 6, "Failed to initialize."),
        ( 7, "Failed to parse."),
        ( 8, "A Wrong argument is given."),
        ( 9, "A wrong format is given."),
        (10, "The argument is illegal."),
        (11, "Data not found.")
    ]);
}

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
    WrongArgument = 8,
    WrongFormat = 9,
    IllegalArgument = 10,
    DataNotFound = 11,
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

#[derive(Debug)]
pub enum ErrorStruct<T> {
    Ok(T),
    Err(Error)
}

impl<T> ErrorStruct<T> {
    pub fn default_err(error_type: ErrorType) -> Self {
        Self::Err(Error::default(error_type))
    }

    pub fn error(error_type: ErrorType, description: &str) -> Self {
        Self::Err(Error::new(error_type, description.to_string()))
    }

    pub fn from_result<E>(result: Result<T, E>) -> Self {
        match result {
            Ok(ok) => Self::Ok(ok),
            Err(_) => Self::default_err(ErrorType::UnknownProblem)
        }
    }

    pub fn from_result_by_to_string<E: ToString>(result: Result<T, E>) -> Self {
        match result {
            Ok(ok) => Self::Ok(ok),
            Err(e) => Self::Err(Error::new(ErrorType::UnknownProblem, e.to_string()))
        }
    }

    pub fn from_result_by_display<E: std::fmt::Display>(result: Result<T, E>) -> Self {
        match result {
            Ok(ok) => Self::Ok(ok),
            Err(e) => Self::Err(Error::new(ErrorType::UnknownProblem, format!("{}", e)))
        }
    }

    pub fn from_result_by_debug<E: std::fmt::Debug>(result: Result<T, E>) -> Self {
        match result {
            Ok(ok) => Self::Ok(ok),
            Err(e) => Self::Err(Error::new(ErrorType::UnknownProblem, format!("{:?}", e)))
        }
    }

    pub fn from_option(option: Option<T>) -> Self {
        match option {
            Some(ok) => Self::Ok(ok),
            None => Self::default_err(ErrorType::DataNotFound)
        }
    }

    pub fn ok_work<U>(&self, f: &dyn Fn(&T) -> U) -> ErrorStruct<U> {
        match self {
            Self::Ok(ok) => ErrorStruct::Ok(f(ok)),
            Self::Err(err) => ErrorStruct::Err(err.clone())
        }
    }

    pub fn ok_work_mut<U>(&mut self, f: &dyn Fn(&mut T) -> U) -> ErrorStruct<U> {
        match self {
            Self::Ok(ok) => ErrorStruct::Ok(f(ok)),
            Self::Err(err) => ErrorStruct::Err(err.clone())
        }
    }

    pub fn ok_work_with_exception<U>(&self, f: &dyn Fn(&T) -> ErrorStruct<U>) -> ErrorStruct<U> {
        match self {
            Self::Ok(ok) => match f(ok) {
                ErrorStruct::Ok(ok_ok) => ErrorStruct::Ok(ok_ok),
                ErrorStruct::Err(ok_e) => ErrorStruct::Err(ok_e.clone())
            },
            Self::Err(err) => ErrorStruct::Err(err.clone())
        }
    }

    pub fn to_result(self) -> Result<T, Error> {
        match self {
            Self::Ok(ok) => Ok(ok),
            Self::Err(err) => Err(err)
        }
    }

    pub fn unwrap(self) -> T {
        self.to_result().unwrap()
    }
 
    pub fn unwrap_or(self, default: T) -> T {
        self.to_result().unwrap_or(default)
    }

    pub fn expect(self, msg: &str) -> T {
        self.to_result().expect(msg)
    }
}