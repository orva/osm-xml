extern crate xml;

use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum Error {
    BoundsMissing(ErrorReason),
    MalformedTag(ErrorReason),
    MalformedNode(ErrorReason),
    MalformedWay(ErrorReason),
    MalformedRelation(ErrorReason),
    UnknownElement,
    XmlParseError(xml::reader::Error)
}

#[derive(Debug)]
pub enum ErrorReason {
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    IllegalNesting,
    Missing
}

impl From<ParseFloatError> for ErrorReason {
    fn from(err: ParseFloatError) -> ErrorReason {
        ErrorReason::ParseFloat(err)
    }
}

impl From<ParseIntError> for ErrorReason {
    fn from(err: ParseIntError) -> ErrorReason {
        ErrorReason::ParseInt(err)
    }
}

impl From<xml::reader::Error> for Error {
    fn from(err: xml::reader::Error) -> Error {
        Error::XmlParseError(err)
    }
}


