use core::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};    // Errors have to apply both the Debug and Display Traits
use std::str::Utf8Error;
use super::method::{Method, MethodError};
use super::{QueryString};

pub struct Request<'buf> {  // LA The struct Request has a lifetime parameter 'buf, which indicates that any reference within this struct will live at least as long as 'buf.
    path: &'buf str,        // LA The path field is a reference to a string slice with the lifetime 'buf. This means the path reference cannot outlive the data it points to.
    query_string: Option<QueryString<'buf>>, // The query_string is an optional reference to a string slice with the same lifetime 'buf. It ensures that if query_string is Some, the reference is valid for the lifetime 'buf.
    method: Method,         // LA The method field does not have a lifetime annotation because it is not a reference.
}

// impl Request {
     // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}   // We don't do it this way(directly) because using traits is idiomatic rust and how type conversions are meant to be carried out.
// }

impl<'buf> TryFrom<&'buf[u8]> for Request<'buf> {   // LA The implementation block for TryFrom uses the lifetime 'buf, indicating that the input byte slice and the resulting Request struct share the same lifetime.
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS... - example request for visualisation

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error>{   // LA The function try_from takes a reference to a byte slice with lifetime 'buf. The Result type also uses 'buf to ensure that the returned Request struct's references are valid for the same lifetime.
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
        let request = str::from_utf8(buf)?; // LA Converts the byte slice to a string slice. The resulting &str inherits the lifetime 'buf from buf.

        // match get_next_word(request){    // Returns Option so we have to match on the output
        //     Some((method, request)) => {},
        //     None => return Err((ParseError::InvalidRequest)),
        // }

        // Alternative way to handle this which uses our ? operator as done previously in scope
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;   // We use the ok_or to transform our option output into a result Some => Ok and None => Err // Variable shadowing is used here. request is not a change from the original request variable defined instead it is created anew and the old variable becomes unusable // LA The get_next_word function returns a tuple of string slices, both of which have the same lifetime as the input request.
        // We get our method(/GET) from the request and then output the remainder of the request and then when we run it it separates the next space which is the path (/search?name=abc&sort=1 HTTP/1.1) along with the remainder of the request.
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;    // LA Similarly, path and the remaining request string slice share the same lifetime.
        // Since we are also checking for \r we also get our protocol on a 3rd call of this function
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;  // LA The protocol string slice also shares the same lifetime.

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;  // Parses the method string slice into a Method type. No lifetime annotation is needed here as Method is not a reference. // Can't use ? because this function is expecting a parse error but our method returns a Method Error therefore we must convert 

        let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);    // query_string previously defined as an option type
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // Alternative method of peforming commented code above
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));    // LA The query_string is a slice of path, so it naturally inherits the lifetime 'buf.
            path = &path[..i];  // LA The path is updated to a slice of itself, maintaining the same lifetime 'buf.
        }

        Ok(Self {
            path,            // LA The path field is set with a reference that has the lifetime 'buf.
            query_string,    // LA The query_string field is set with a reference that has the lifetime 'buf.
            method,          // The method field is set without a lifetime annotation as it is not a reference.
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {   // LA The function get_next_word takes a string slice and returns an Option containing a tuple of two string slices. All these slices have the same lifetime as the input request.
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break 
    //     }
    // }

    // The above code can be written alternatively using this format
    for (i, c) in request.chars().enumerate() {  // Loop through each character in the request string. 'enumerate' provides both the index (i) and the character (c) for each iteration.
        if c == ' ' || c == '\r'{  // Check if the current character is a space.
            return Some((&request[..i], &request[i + 1..]));  // If a space is found, return a tuple. The first part is the substring before the space, and the second part is the substring after the space. // LA Returns a tuple of two slices of the input request, both sharing the same lifetime as the input.
            // This code is safe because it deals with UTF-8 characters correctly. If we were dealing with multi-byte characters (like emojis) incorrectly, it could lead to invalid UTF-8 and potentially crash the program.
        }
    }
    
    None
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_:Utf8Error) -> Self {
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