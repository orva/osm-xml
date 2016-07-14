extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::num;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

pub type Coordinate = f64;

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub key: String,
    pub val: String
}

#[derive(Debug, PartialEq)]
pub struct Bounds {
    pub minlat: Coordinate,
    pub minlon: Coordinate,
    pub maxlat: Coordinate,
    pub maxlon: Coordinate,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: i64,
    pub lat: Coordinate,
    pub lon: Coordinate,
    pub tags: Vec<Tag>
}

#[derive(Debug)]
pub struct OSM {
    pub bounds: Option<Bounds>,
    pub nodes: Vec<Node>
}

impl OSM {
    fn empty() -> OSM {
        OSM {
            bounds: None,
            nodes: Vec::new(),
        }
    }

    pub fn parse(file: File) -> Option<OSM> {
        let mut osm = OSM::empty();

        let reader = BufReader::new(file);
        let parser = EventReader::new(reader);

        for element in parser {
            match element {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    match ElementType::from_str(&name.local_name) {
                        Ok(element) => {
                            match element {
                                ElementType::Bounds => set_bounds(&mut osm, &attributes),
                                ElementType::Node => insert_node(&mut osm, &attributes),
                                _ => ()
                            }
                        },
                        Err(_) => ()
                    }
                },
                _ => { }
            }
        }

        Some(osm)
    }
}

#[derive(Debug)]
enum ErrorKind {
    BoundsMissing(AttributeError),
    IdMissing(AttributeError),
    CoordinateMissing(AttributeError),
    UnknownElement
}

#[derive(Debug)]
enum AttributeError {
    ParseFloat(num::ParseFloatError),
    ParseInt(num::ParseIntError),
    Missing
}

impl From<num::ParseFloatError> for AttributeError {
    fn from(err: num::ParseFloatError) -> AttributeError {
        AttributeError::ParseFloat(err)
    }
}

impl From<num::ParseIntError> for AttributeError {
    fn from(err: num::ParseIntError) -> AttributeError {
        AttributeError::ParseInt(err)
    }
}

#[allow(dead_code)]
enum ElementType {
    Bounds,
    Node,
    Way,
    Relation
}


impl FromStr for ElementType {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<ElementType, ErrorKind> {
        let downcased = s.to_lowercase();

        if downcased == "bounds" {
            return Ok(ElementType::Bounds);
        }

        if downcased == "node" {
            return Ok(ElementType::Node);
        }

        if downcased == "way" {
            return Ok(ElementType::Way);
        }

        if downcased == "relation" {
            return Ok(ElementType::Relation);
        }

        Err(ErrorKind::UnknownElement)
    }
}

fn set_bounds(osm: &mut OSM, attrs: &Vec<OwnedAttribute>) {
    match parse_bounds(&attrs) {
        Ok(bounds) => osm.bounds = Some(bounds),
        Err(_) => osm.bounds = None
    }
}

fn insert_node(osm: &mut OSM, attrs: &Vec<OwnedAttribute>) {
    match parse_node(&attrs) {
        Ok(node) => osm.nodes.push(node),
        Err(_) => ()
    }
}

fn parse_bounds(attrs: &Vec<OwnedAttribute>) -> Result<Bounds, ErrorKind> {
    let minlat = try!(find_attribute("minlat", attrs).map_err(ErrorKind::BoundsMissing));
    let minlon = try!(find_attribute("minlon", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlat = try!(find_attribute("maxlat", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlon = try!(find_attribute("maxlon", attrs).map_err(ErrorKind::BoundsMissing));

    Ok(Bounds {
        minlat: minlat,
        minlon: minlon,
        maxlat: maxlat,
        maxlon: maxlon,
    })
}

fn parse_node(attrs: &Vec<OwnedAttribute>) -> Result<Node, ErrorKind> {
    let id = try!(find_attribute("id", attrs).map_err(ErrorKind::IdMissing));
    let lon = try!(find_attribute("lon", attrs).map_err(ErrorKind::CoordinateMissing));
    let lat = try!(find_attribute("lat", attrs).map_err(ErrorKind::CoordinateMissing));

    Ok(Node {
        id: id,
        lon: lon,
        lat: lat,
        tags: Vec::new()
    })
}

fn find_attribute<T>( name: &str, attrs: &Vec<OwnedAttribute>) -> Result<T, AttributeError>
    where AttributeError: std::convert::From<<T as std::str::FromStr>::Err>,
          T: FromStr
{
    let attr = attrs.iter().find(|attr| attr.name.local_name == name);
    match attr {
        Some(a) => {
            let val = try!(a.value.parse::<T>());
            Ok(val)
        },
        None => Err(AttributeError::Missing)
    }
}
