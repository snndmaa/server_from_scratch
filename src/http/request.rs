use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};    // Errors have to apply both the Debug and Display Traits

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}   // We don't do it this way because using traits is idiomatic rust and how type conversions are meant to be carried out.
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1 - example request

    fn try_from(value: &[u8]) -> Result<Self, Self::Error>{
        unimplemented!()
    }
}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
            
        }
    }
}

impl Display for ParseError {  // Important for our ParseError to display on console like the Errors in the std
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {  // Important for our ParseError to debug {:?} on console like the Errors in the std
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}