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
                        ElementData::Node(id, lat, lon, tags) => {
                            osm.nodes.push(Node {
                                id: id,
                                lat: lat,
                                lon: lon,
                                tags: tags
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
    NodeAttrs(i64, f64, f64),
    Node(i64, f64, f64, Vec<Tag>),
    Tags(Vec<Tag>),
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

        if downcased == "tag" {
            return Ok(ElementType::Tag);
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
                ElementType::Node => parse_node(parser, &attributes),
                _ => Err(ErrorKind::UnknownElement)
            }
        },
        _ => Ok(ElementData::Ignored)
    }
}

fn parse_node(parser: &mut EventReader<BufReader<File>>, attrs: &Vec<OwnedAttribute>) -> Result<ElementData, ErrorKind> {
    let node_attrs = try!(parse_node_attributes(&attrs));
    let wrapped_tags = try!(parse_tags(parser));

    match node_attrs {
        ElementData::NodeAttrs(id, lat, lon) => {
            let tags = match wrapped_tags {
                ElementData::Tags(tag_arr) => tag_arr,
                _ => Vec::new()
            };

            Ok(ElementData::Node(id, lat, lon, tags))
        },
        _ => Err(ErrorKind::UnknownElement)
    }
}

fn parse_tags(parser: &mut EventReader<BufReader<File>>) -> Result<ElementData, ErrorKind> {
    let mut tags = Vec::new();

    loop {
        let element = try!(parser.next());

        match element {
            XmlEvent::EndElement { name } => {
                let element_type = try!(ElementType::from_str(&name.local_name));

                match element_type {
                    ElementType::Tag => continue,
                    _ => return Ok(ElementData::Tags(tags))
                }
            },
            XmlEvent::StartElement { name, attributes, .. } => {
                let element_type = try!(ElementType::from_str(&name.local_name));

                match element_type {
                    ElementType::Tag => {
                        if let Some(tag) = parse_tag_attributes(&attributes) {
                            tags.push(tag);
                        }
                    },
                    _ => continue
                }
            },
            _ => continue
        }
    }
}

fn parse_tag_attributes(attributes: &Vec<OwnedAttribute>) -> Option<Tag> {
    let mut iter = attributes.iter();
    iter.find(|attr| attr.name.local_name == "k")
        .and_then(|attr| Some(attr.value.clone()))
        .and_then(|key| {
            iter.find(|attr| attr.name.local_name == "v")
            .and_then(|attr| Some(attr.value.clone()))
            .and_then(|val| Some(Tag { key: key, val: val }))
        })
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

    Ok(ElementData::NodeAttrs(id, lat, lon))
}

fn find_attribute<T>(name: &str, attrs: &Vec<OwnedAttribute>) -> Result<T, AttributeError>
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
