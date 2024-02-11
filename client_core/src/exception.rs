use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref STANDARD_ERROR_DESCRIPTION: HashMap<isize, &'static str> = HashMap::from([
        (0, "Wrong format is given"),
        (1, "Cannot convert the object.")
    ]);

    pub static ref STANDARD_WARN_DESCRIPTION: HashMap<isize, &'static str> = HashMap::from([
        (0, "Cannot find the id.")
    ]);
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    WrongFormat = 0,
    ConvertFiled = 1,
}

#[derive(Debug)]
pub struct Error {
    error: ErrorType,
    description: String,
}

impl Error {
    pub fn new(error: ErrorType, description: String) -> Self {
        Self { error, description }
    }

    pub fn default(error: ErrorType) -> Self {
        let error_id = error.clone() as isize;
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

// #[unstable(feature = "try_trait_v2")]
// impl<T> Try for ErrorStruct<T> {
//     type Output = T;
//     type Residual = Result<std::convert::Infallible, Error>;

//     fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
//         match self {
//             Self::Ok(v) => ControlFlow::Continue(v),
//             Self::Err(e) => ControlFlow::Break(Err(e)),
//         }
//     }

//     fn from_output(output: Self::Output) -> Self {
//         Self::Ok(output)
//     }
// }

#[derive(Clone, Debug)]
pub enum WarnType {
    IdNotFound = 0
}

pub struct Warn {
    warn: WarnType,
    description: String,
}

impl Warn {
    pub fn new(warn: WarnType, description: String) -> Self {
        Self { warn, description }
    }

    pub fn default(warn: WarnType) -> Self {
        let warn_id = warn.clone() as isize;
        let description = STANDARD_WARN_DESCRIPTION.get(&warn_id).unwrap();
        Self::new(warn, description.to_string())
    }
}

impl std::fmt::Display for Warn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Warn<warn<{:?}>,description<{}>>", self.warn, self.description)
    }
}
