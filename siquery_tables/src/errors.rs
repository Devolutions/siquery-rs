use std::error::Error;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ProxyError {
    InvalidConfigError,
    NoProxyConfiguredError,
    OsError,
}

use ProxyError::*;
pub type Result<T> = ::std::result::Result<T, ProxyError>;

impl Error for ProxyError {
    fn description(&self) -> &str {
        match *self {
            InvalidConfigError => "invalid proxy configuration",
            NoProxyConfiguredError => "no proxy configuration found",
            OsError => "error getting proxy configuration from the Operating System",
        }
    }
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => self.description().fmt(f),
        }
    }
}

impl From<::url::ParseError> for ProxyError {
    fn from(_error: ::url::ParseError) -> Self {
        InvalidConfigError
    }
}
