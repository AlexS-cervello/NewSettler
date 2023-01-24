use std::env::VarError;

use crate::migration::DbErr;
use log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors;
use teloxide::RequestError;

#[derive(Debug)]
pub enum Error {
    Database(DbErr),
    File(std::io::Error),
    Generic(Box<dyn std::error::Error + Send + Sync>),
    TeloxideRequest(RequestError),
    Request(reqwest::Error),
    Config(ConfigErrors),
    Var(VarError),
    Logger(SetLoggerError),
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
            Self::Config(ref err) => {
                write!(f, "Config error: {}", err)
            }
            Self::Var(ref err) => {
                write!(f, "Variable error: {}", err)
            }
            Self::Logger(ref err) => {
                write!(f, "Logger error: {}", err)
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

impl From<ConfigErrors> for Error {
    fn from(err: ConfigErrors) -> Self {
        Self::Config(err)
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Self::Var(err)
    }
}

impl From<SetLoggerError> for Error {
    fn from(err: SetLoggerError) -> Self {
        Self::Logger(err)
    }
}
