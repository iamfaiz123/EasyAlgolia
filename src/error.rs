///! Error from EasyAlgolia
use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ErrorKind {
    RequestError,
    ClientBuilderError,
    ClientError
}

pub struct EasyAlgoliaError {
    error_kind: ErrorKind,
    cause: Option<String>,
}

impl fmt::Debug for EasyAlgoliaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write custom formatting logic here
        match self.error_kind {
            ErrorKind::RequestError => {
                if let Some(cause) = self.cause.as_ref() {
                    write!(
                        f,
                        "EasyAlgoliaError {{ error_kind: {:?}, cause: {:?} }}",
                        self.error_kind,
                        cause.replace("\"", "")
                    )
                } else {
                    write!(f, "EasyAlgoliaError {{ error_kind: {:?}}}", self.error_kind)
                }
            }
            _ => {
                write!(
                    f,
                    "EasyAlgoliaError {{ error_kind: {:?}, cause: {:?} }}",
                    self.error_kind, self.cause
                )
            }
        }
    }
}

impl From<reqwest::Error> for EasyAlgoliaError {
    fn from(_: reqwest::Error) -> Self {
        EasyAlgoliaError::new(ErrorKind::RequestError, None)
    }
}
impl<'a> EasyAlgoliaError {
    pub(crate) fn new(error_kind: ErrorKind, cause: Option<String>) -> Self {
        match error_kind {
            ErrorKind::ClientBuilderError => Self { error_kind, cause },
            ErrorKind::RequestError => Self { error_kind, cause },
            ErrorKind::ClientError => Self { error_kind, cause }
        }
    }
}

impl<'a> fmt::Display for EasyAlgoliaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement formatting of your error message here
        write!(f, "An error occurred in EasyAlgolia: ...")
    }
}

impl Error for EasyAlgoliaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Optionally provide the underlying cause of the error
        None
    }
}
