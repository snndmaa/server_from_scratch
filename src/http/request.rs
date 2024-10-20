use super::method::Method;
use core::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};    // Errors have to apply both the Debug and Display Traits
use std::str::Utf8Error;

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

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }

        // Above error handling comment can be written alternatively as:
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {    // This returns the res:Err if the result is an error if nto it returns the Ok value of self
        //     Ok(request) => {}
        //     Err(e) => Err(e)
        // }

        // This in turn can be written in shorthand as the following. Only difference is this will try to convert the error type if it does not match the initally specified type:
        let request = str::from_utf8(buf)?;

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

impl From<Utf8Error> for ParseError {
    fn from(value: T) -> Self {
        Self::InvalidEncoding
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