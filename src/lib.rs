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
pub type Role = String;

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
    pub id: Id,
    pub lat: Coordinate,
    pub lon: Coordinate,
    pub tags: Vec<Tag>
}

#[derive(Debug, PartialEq)]
pub struct Way {
    pub id: Id,
    pub tags: Vec<Tag>,
    pub nodes: Vec<UnresolvedReference>
}

#[derive(Debug, PartialEq)]
pub struct Relation {
    pub id: Id,
    pub members: Vec<Member>,
    pub tags: Vec<Tag>
}

#[derive(Debug, PartialEq)]
pub enum Member {
    Node(UnresolvedReference, Role),
    Way(UnresolvedReference, Role),
    Relation(UnresolvedReference, Role),
}

#[derive(Debug, PartialEq)]
pub enum UnresolvedReference {
    Node(Id),
    Way(Id),
    Relation(Id)
}

#[derive(Debug, PartialEq)]
pub enum Reference<'a> {
    Node(&'a Node),
    Way(&'a Way),
    Relation(&'a Relation),
    Unresolved
}

#[derive(Debug)]
pub struct OSM {
    pub bounds: Option<Bounds>,
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,
    pub relations: Vec<Relation>
}

impl OSM {
    fn empty() -> OSM {
        OSM {
            bounds: None,
            nodes: Vec::new(),
            ways: Vec::new(),
            relations: Vec::new(),
        }
    }

    pub fn parse(file: File) -> Result<OSM, ErrorKind> {
        let mut osm = OSM::empty();

        let reader = BufReader::new(file);
        let mut parser = EventReader::new(reader);

        loop {
            match parse_element_data(&mut parser) {
                Err(ErrorKind::XmlParseError(err)) => return Err(ErrorKind::XmlParseError(err)),
                Err(ErrorKind::BoundsMissing(_)) => osm.bounds = None,
                Err(ErrorKind::MalformedTag(_))       |
                Err(ErrorKind::MalformedNode(_))      |
                Err(ErrorKind::MalformedWay(_))       |
                Err(ErrorKind::MalformedRelation(_))  |
                Err(ErrorKind::UnknownElement)        => continue,
                Ok(data) => {
                    match data {
                        ElementData::EndOfDocument => return Ok(osm),
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
                        ElementData::Way(id, node_refs, tags) => {
                            osm.ways.push(Way {
                                id: id,
                                nodes: node_refs,
                                tags: tags
                            });
                        },
                        ElementData::Relation(relation) => {
                            osm.relations.push(relation);
                        }
                    }
                }
            }
        }
    }

    pub fn resolve_reference<'a>(&self, reference: &UnresolvedReference) -> Reference {
        match *reference {
            UnresolvedReference::Node(id) => {
                match self.nodes.iter().find(|node| node.id == id) {
                    Some(node) => Reference::Node(&node),
                    None => Reference::Unresolved
                }
            },
            UnresolvedReference::Way(id) => {
                match self.ways.iter().find(|way| way.id == id) {
                    Some(way) => Reference::Way(&way),
                    None => Reference::Unresolved
                }
            },
            UnresolvedReference::Relation(id) => {
                match self.relations.iter().find(|relation| relation.id == id) {
                    Some(relation) => Reference::Relation(&relation),
                    None => Reference::Unresolved
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
    Tag,
    NodeRef,
    Member,
}

enum ElementData {
    Bounds(Coordinate, Coordinate, Coordinate, Coordinate),
    Node(Id, Coordinate, Coordinate, Vec<Tag>),
    Way(Id, Vec<UnresolvedReference>, Vec<Tag>),
    Relation(Relation),
    // These two are here so we can terminate and skip uninteresting data without
    // using error handling.
    EndOfDocument,
    Ignored
}


impl FromStr for ElementType {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<ElementType, ErrorKind> {
        match s.to_lowercase().as_ref() {
            "bounds"   => Ok(ElementType::Bounds),
            "node"     => Ok(ElementType::Node),
            "way"      => Ok(ElementType::Way),
            "relation" => Ok(ElementType::Relation),
            "tag"      => Ok(ElementType::Tag),
            "nd"       => Ok(ElementType::NodeRef),
            "member"   => Ok(ElementType::Member),
            _ => Err(ErrorKind::UnknownElement)
        }
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
                ElementType::Relation => parse_relation(parser, &attributes),
                _ => Err(ErrorKind::UnknownElement)
            }
        },
        _ => Ok(ElementData::Ignored)
    }
}

fn parse_relation(parser: &mut EventReader<BufReader<File>>, attrs: &Vec<OwnedAttribute>) -> Result<ElementData, ErrorKind> {
    let id = try!(find_attribute("id", attrs).map_err(ErrorKind::MalformedRelation));

    let mut members = Vec::new();
    let mut tags = Vec::new();

    loop {
        match try!(parser.next()) {
            XmlEvent::EndElement { name } => {
                let element_type = try!(ElementType::from_str(&name.local_name));

                match element_type {
                    ElementType::Relation => return Ok(ElementData::Relation(Relation {
                        id: id,
                        members: members,
                        tags: tags
                    })),
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
                    ElementType::Member => {
                        let el_type = try!(find_attribute_uncasted("type", &attributes).map_err(ErrorKind::MalformedRelation));
                        let el_ref = try!(find_attribute("ref", &attributes).map_err(ErrorKind::MalformedRelation));
                        let el_role = try!(find_attribute_uncasted("role", &attributes).map_err(ErrorKind::MalformedRelation));

                        let el = match el_type.to_lowercase().as_ref() {
                            "node" => Member::Node(UnresolvedReference::Node(el_ref), el_role),
                            "way" => Member::Way(UnresolvedReference::Way(el_ref), el_role),
                            "relation" => Member::Relation(UnresolvedReference::Relation(el_ref), el_role),
                            _ => return Err(ErrorKind::MalformedRelation(AttributeError::Missing))
                        };

                        members.push(el);
                    },
                    ElementType::Bounds   |
                    ElementType::Node     |
                    ElementType::Relation |
                    ElementType::Way      |
                    ElementType::NodeRef  => return Err(
                        ErrorKind::MalformedRelation(AttributeError::IllegalNesting)
                    )
                }
            },
            _ => continue
        }
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
                        node_refs.push(UnresolvedReference::Node(node_ref));
                    },
                    ElementType::Bounds   |
                    ElementType::Node     |
                    ElementType::Relation |
                    ElementType::Way      |
                    ElementType::Member   => return Err(
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
                    ElementType::Bounds   |
                    ElementType::Node     |
                    ElementType::Relation |
                    ElementType::Way      |
                    ElementType::NodeRef  |
                    ElementType::Member   => return Err(
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
