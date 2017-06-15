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
    XmlParseError(xml::reader::Error),
}

use std::fmt;

#[derive(Debug)]
pub enum ErrorReason {
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    IllegalNesting,
    Missing,
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match *self {
            BoundsMissing(ref reason) => write!(f, "OSM XML error: Missing bounds: {:?}", reason),
            MalformedTag(ref reason) => write!(f, "OSM XML error: Malformed tag: {:?}", reason),
            MalformedNode(ref reason) => write!(f, "OSM XML error: Malformed node: {:?}", reason),
            MalformedWay(ref reason) => write!(f, "OSM XML error: Malformed way: {:?}", reason),
            MalformedRelation(ref reason) => write!(f, "OSM XML error: Malformed relation: {:?}", reason),
            UnknownElement => write!(f, "OSM XML error: Unknown XML element"),
            XmlParseError(ref reason) => write!(f, "OSM XML parse error: {}", reason),
        }
        
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "OSM XML error"
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        None
    }
}