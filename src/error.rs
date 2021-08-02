use std::io;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum SunshineError {
  IOError(io::Error),
  MessageError(String)
}

pub type Result<T> = result::Result<T, SunshineError>;

impl fmt::Display for SunshineError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SunshineError::IOError(error) => write!(f, "IO Error: {}", error),
      SunshineError::MessageError(message) => write!(f, "{}", message),
    }
  }
}

impl error::Error for SunshineError {

  fn cause(&self) -> Option<&dyn error::Error> {
    match self {
      SunshineError::IOError(ref error) => Some(error),
      SunshineError::MessageError(_message) => None,
    }
  }
}


impl From<io::Error> for SunshineError {
  fn from(error: io::Error) -> Self {
    SunshineError::IOError(error)
  }
}

impl From<&str> for SunshineError {
  fn from(message: &str) -> Self {
    SunshineError::MessageError(message.to_string())
  }
}