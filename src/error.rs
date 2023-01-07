use crate::migration::DbErr;
use teloxide::RequestError;

#[derive(Debug)]
pub enum Error {
    Database(DbErr),
    File(std::io::Error),
    Generic(Box<dyn std::error::Error + Send + Sync>),
    TeloxideRequest(RequestError),
    Request(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Database(ref err) => {
                write!(f, "Database error: {}", err)
            }
            Self::File(ref err) => {
                write!(f, "File error: {}", err)
            }
            Self::Generic(ref err) => {
                write!(f, "Other error: {}", err)
            }
            Self::TeloxideRequest(ref err) => {
                write!(f, "Teloxide request error: {}", err)
            }
            Self::Request(ref err) => {
                write!(f, "Request error: {}", err)
            }
        }
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Self::Database(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::File(err)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Generic(err)
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Self {
        Self::TeloxideRequest(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}
