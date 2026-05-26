use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    HttpError(reqwest::Error),
    CityNotFound,
    InputError,
}
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::HttpError(e)
    }
}
impl Error for AppError {}