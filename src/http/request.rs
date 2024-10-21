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

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();

    loop {
        let item = iter.next();
        match item {
            Some(c) => {}
            None => break 
        }
    }

    for (i, c) in request.chars().enumerate() {  // Loop through each character in the request string. 'enumerate' provides both the index (i) and the character (c) for each iteration.
    if c == ' ' {  // Check if the current character is a space.
        return Some((&request[..i], &request[i + 1..]));  // If a space is found, return a tuple. The first part is the substring before the space, and the second part is the substring after the space.
        // This code is safe because it deals with UTF-8 characters correctly. If we were dealing with multi-byte characters (like emojis) incorrectly, it could lead to invalid UTF-8 and potentially crash the program.
    }
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