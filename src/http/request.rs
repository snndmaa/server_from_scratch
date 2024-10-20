use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}   // We don't do it this way because using traits is idiomatic rust and how type conversions are meant to be carried out.
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error>{
        unimplemented!()
    }
}