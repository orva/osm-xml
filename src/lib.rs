extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

mod error;
use error::{ErrorKind, AttributeError};


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
        let mut parser = EventReader::new(reader);

        loop {
            match parse_element_data(&mut parser) {
                Err(ErrorKind::UnknownElement) => continue,
                Err(ErrorKind::BoundsMissing(_)) => osm.bounds = None,
                Err(_) => return None,
                Ok(data) => {
                    match data {
                        ElementData::EndOfDocument => return Some(osm),
                        ElementData::Ignored => continue,
                        ElementData::Bounds(minlat, minlon, maxlat, maxlon) => {
                            osm.bounds = Some(Bounds {
                                minlat: minlat,
                                minlon: minlon,
                                maxlat: maxlat,
                                maxlon: maxlon
                            });
                        },
                        ElementData::Node(id, lat, lon) => {
                            osm.nodes.push(Node {
                                id: id,
                                lat: lat,
                                lon: lon,
                                tags: Vec::new()
                            });
                        },
                        _ => ()
                    }
                }
            }
        }
    }
}

enum ElementType {
    Bounds,
    Node,
    Way,
    Relation,
    Tag
}

enum ElementData {
    Bounds(f64, f64, f64, f64),
    Node(i64, f64, f64),
    Tag(String, String),
    // These two are here so we can terminate and skip uninteresting data without
    // using error handling.
    EndOfDocument,
    Ignored
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

fn parse_element_data(parser: &mut EventReader<BufReader<File>>) -> Result<ElementData, ErrorKind> {
    let element = try!(parser.next());
    match element {
        XmlEvent::EndDocument => Ok(ElementData::EndOfDocument),
        XmlEvent::StartElement { name, attributes, .. } => {
            let element_type = try!(ElementType::from_str(&name.local_name));
            match element_type {
                ElementType::Bounds => parse_bounds(&attributes),
                ElementType::Node => parse_node_attributes(&attributes),
                _ => Err(ErrorKind::UnknownElement)
            }
        },
        _ => Ok(ElementData::Ignored)
    }
}

fn parse_bounds(attrs: &Vec<OwnedAttribute>) -> Result<ElementData, ErrorKind> {
    let minlat = try!(find_attribute("minlat", attrs).map_err(ErrorKind::BoundsMissing));
    let minlon = try!(find_attribute("minlon", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlat = try!(find_attribute("maxlat", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlon = try!(find_attribute("maxlon", attrs).map_err(ErrorKind::BoundsMissing));

    Ok(ElementData::Bounds(minlat, minlon, maxlat, maxlon))
}

fn parse_node_attributes(attrs: &Vec<OwnedAttribute>) -> Result<ElementData, ErrorKind> {
    let id = try!(find_attribute("id", attrs).map_err(ErrorKind::IdMissing));
    let lat = try!(find_attribute("lat", attrs).map_err(ErrorKind::CoordinateMissing));
    let lon = try!(find_attribute("lon", attrs).map_err(ErrorKind::CoordinateMissing));

    Ok(ElementData::Node(id, lat, lon))
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
