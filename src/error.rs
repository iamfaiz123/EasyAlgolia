///! Error from EasyAlgolia
use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ErrorKind {
    RequestError,
    ClientBuilderError,
}
#[derive(Debug)]
pub struct EasyAlgoliaError<'a> {
    error_kind: ErrorKind,
    cause: Option<&'a str>,
}
impl<'a> EasyAlgoliaError<'a> {
    pub(crate) fn new(error_kind: ErrorKind, cause: Option<&'a str>) -> Self {
        match error_kind {
            ErrorKind::ClientBuilderError => Self { error_kind, cause },
            _ => {
                todo!()
            }
        }
    }
}

impl<'a> fmt::Display for EasyAlgoliaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement formatting of your error message here
        write!(f, "An error occurred in EasyAlgolia: ...")
    }
}

impl<'a> Error for EasyAlgoliaError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Optionally provide the underlying cause of the error
        None
    }
}
