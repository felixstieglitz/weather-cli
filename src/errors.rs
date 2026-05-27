use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    HttpError,
    CityNotFound,
}
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> Self {
        AppError::HttpError
    }
}
impl Error for AppError {}