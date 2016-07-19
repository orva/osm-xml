extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

mod error;
use error::{ErrorKind, AttributeError};


pub type Coordinate = f64;
pub type Id = i64;

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

#[derive(Debug, PartialEq)]
pub struct Way {
    pub id: i64,
    pub tags: Vec<Tag>,
    node_ids: Vec<Id>
}

#[derive(Debug)]
pub struct OSM {
    pub bounds: Option<Bounds>,
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>
}

impl OSM {
    fn empty() -> OSM {
        OSM {
            bounds: None,
            nodes: Vec::new(),
            ways: Vec::new()
        }
    }

    pub fn parse(file: File) -> Option<OSM> {
        let mut osm = OSM::empty();

        let reader = BufReader::new(file);
        let mut parser = EventReader::new(reader);

        loop {
            match parse_element_data(&mut parser) {
                Err(ErrorKind::XmlParseError(_)) => return None,
                Err(ErrorKind::BoundsMissing(_)) => osm.bounds = None,
                Err(ErrorKind::MalformedTag(_))  |
                Err(ErrorKind::MalformedNode(_)) |
                Err(ErrorKind::MalformedWay(_))  |
                Err(ErrorKind::UnknownElement)   => continue,
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
                        ElementData::Way(id, node_ids, tags) => {
                            osm.ways.push(Way {
                                id: id,
                                node_ids: node_ids,
                                tags: tags
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn nodes_for(&self, way: &Way) -> Vec<&Node> {
        way.node_ids.iter()
            .map(|id| self.nodes.iter().find(|node| node.id == *id).unwrap())
            .collect()
    }
}

enum ElementType {
    Bounds,
    Node,
    Way,
    Relation,
    Tag,
    NodeRef,
}

enum ElementData {
    Bounds(Coordinate, Coordinate, Coordinate, Coordinate),
    Node(Id, Coordinate, Coordinate, Vec<Tag>),
    Way(Id, Vec<Id>, Vec<Tag>),
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

        if downcased == "nd" {
            return Ok(ElementType::NodeRef);
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
                ElementType::Way => parse_way(parser, &attributes),
                _ => Err(ErrorKind::UnknownElement)
            }
        },
        _ => Ok(ElementData::Ignored)
    }
}

fn parse_way(parser: &mut EventReader<BufReader<File>>, attrs: &Vec<OwnedAttribute>) -> Result<ElementData, ErrorKind> {
    let id = try!(find_attribute("id", attrs).map_err(ErrorKind::MalformedWay));

    let mut node_refs = Vec::new();
    let mut tags = Vec::new();

    loop {
        match try!(parser.next()) {
            XmlEvent::EndElement { name } => {
                let element_type = try!(ElementType::from_str(&name.local_name));

                match element_type {
                    ElementType::Way => return Ok(ElementData::Way(id, node_refs, tags)),
                    _ => continue
                }
            },
            XmlEvent::StartElement { name, attributes, .. } => {
                let element_type = try!(ElementType::from_str(&name.local_name));

                match element_type {
                    ElementType::Tag => {
                        if let Ok(tag) = parse_tag(&attributes) {
                            tags.push(tag);
                        }
                    },
                    ElementType::NodeRef => {
                        let node_ref = try!(find_attribute("ref", &attributes).map_err(ErrorKind::MalformedWay));
                        node_refs.push(node_ref);
                    },
                    ElementType::Bounds |
                    ElementType::Node |
                    ElementType::Relation |
                    ElementType::Way => return Err(
                        ErrorKind::MalformedWay(AttributeError::IllegalNesting)
                    )
                }
            },
            _ => continue
        }
    }

}

fn parse_node(parser: &mut EventReader<BufReader<File>>, attrs: &Vec<OwnedAttribute>) -> Result<ElementData, ErrorKind> {
    let id = try!(find_attribute("id", attrs).map_err(ErrorKind::MalformedNode));
    let lat = try!(find_attribute("lat", attrs).map_err(ErrorKind::MalformedNode));
    let lon = try!(find_attribute("lon", attrs).map_err(ErrorKind::MalformedNode));

    let mut tags = Vec::new();

    loop {
        match try!(parser.next()) {
            XmlEvent::EndElement { name } => {
                let element_type = try!(ElementType::from_str(&name.local_name));

                match element_type {
                    ElementType::Node => return Ok(ElementData::Node(id, lat, lon, tags)),
                    _ => continue
                }
            },
            XmlEvent::StartElement { name, attributes, .. } => {
                let element_type = try!(ElementType::from_str(&name.local_name));

                match element_type {
                    ElementType::Tag => {
                        if let Ok(tag) = parse_tag(&attributes) {
                            tags.push(tag);
                        }
                    },
                    ElementType::Bounds |
                    ElementType::Node |
                    ElementType::Relation |
                    ElementType::Way |
                    ElementType::NodeRef => return Err(
                        ErrorKind::MalformedNode(AttributeError::IllegalNesting)
                    )
                }
            },
            _ => continue
        }
    }
}

fn parse_tag(attributes: &Vec<OwnedAttribute>) -> Result<Tag, ErrorKind> {
    let key = try!(find_attribute_uncasted("k", attributes).map_err(ErrorKind::MalformedTag));
    let val = try!(find_attribute_uncasted("v", attributes).map_err(ErrorKind::MalformedTag));
    Ok(Tag { key: key, val: val })
}

fn parse_bounds(attrs: &Vec<OwnedAttribute>) -> Result<ElementData, ErrorKind> {
    let minlat = try!(find_attribute("minlat", attrs).map_err(ErrorKind::BoundsMissing));
    let minlon = try!(find_attribute("minlon", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlat = try!(find_attribute("maxlat", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlon = try!(find_attribute("maxlon", attrs).map_err(ErrorKind::BoundsMissing));

    Ok(ElementData::Bounds(minlat, minlon, maxlat, maxlon))
}

fn find_attribute<T>(name: &str, attrs: &Vec<OwnedAttribute>) -> Result<T, AttributeError>
    where AttributeError: std::convert::From<<T as std::str::FromStr>::Err>,
          T: FromStr
{
    let val_raw = try!(find_attribute_uncasted(name, attrs));
    let val = try!(val_raw.parse::<T>());
    Ok(val)
}

fn find_attribute_uncasted(name: &str, attrs: &Vec<OwnedAttribute>) -> Result<String, AttributeError> {
    let attr = attrs.iter().find(|attr| attr.name.local_name == name);
    match attr {
        Some(a) => {
            Ok(a.value.clone())
        },
        None => Err(AttributeError::Missing)
    }
}
