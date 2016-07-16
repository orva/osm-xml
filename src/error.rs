extern crate xml;

use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum ErrorKind {
    BoundsMissing(AttributeError),
    IdMissing(AttributeError),
    CoordinateMissing(AttributeError),
    UnknownElement,
    XmlParseError(xml::reader::Error)
}

#[derive(Debug)]
pub enum AttributeError {
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    Missing
}

impl From<ParseFloatError> for AttributeError {
    fn from(err: ParseFloatError) -> AttributeError {
        AttributeError::ParseFloat(err)
    }
}

impl From<ParseIntError> for AttributeError {
    fn from(err: ParseIntError) -> AttributeError {
        AttributeError::ParseInt(err)
    }
}

impl From<xml::reader::Error> for ErrorKind {
    fn from(err: xml::reader::Error) -> ErrorKind {
        ErrorKind::XmlParseError(err)
    }
}


