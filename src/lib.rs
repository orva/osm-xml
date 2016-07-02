extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::num;

use xml::reader::{EventReader, XmlEvent};
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;

pub type Coordinate = f64;

#[derive(Debug, PartialEq)]
pub struct Bounds {
    pub minlat: Coordinate,
    pub minlon: Coordinate,
    pub maxlat: Coordinate,
    pub maxlon: Coordinate,
}

pub struct OSM {
    pub bounds: Option<Bounds>
}

impl OSM {
    fn empty() -> OSM {
        OSM {
            bounds: None
        }
    }

    pub fn parse(file: File) -> Option<OSM> {
        let mut osm = OSM::empty();

        let reader = BufReader::new(file);
        let parser = EventReader::new(reader);

        for element in parser {
            match element {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    handle_element(&mut osm, name, attributes);
                },
                _ => { }
            }
        }

        Some(osm)
    }
}

enum ErrorKind {
    BoundsMissing(AttributeError),
    UnkownElement
}

enum AttributeError {
    ParseFloat(num::ParseFloatError),
    Missing
}

impl From<num::ParseFloatError> for AttributeError {
    fn from(err: num::ParseFloatError) -> AttributeError {
        AttributeError::ParseFloat(err)
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
            return Ok(ElementType::Way);
        }

        if downcased == "way" {
            return Ok(ElementType::Way);
        }

        if downcased == "relation" {
            return Ok(ElementType::Relation);
        }

        Err(ErrorKind::UnkownElement)
    }
}

fn handle_element(osm: &mut OSM, name: OwnedName, attrs: Vec<OwnedAttribute>) {
    match ElementType::from_str(&name.local_name) {
        Ok(element) => {
            match element {
                ElementType::Bounds => set_bounds(osm, &attrs),
                _ => ()
            }
        },
        Err(_) => ()
    }
}

fn set_bounds(osm: &mut OSM, attrs: &Vec<OwnedAttribute>) {
    match parse_bounds(&attrs) {
        Ok(bounds) => osm.bounds = Some(bounds),
        Err(_) => osm.bounds = None
    }
}

fn parse_bounds(attrs: &Vec<OwnedAttribute>) -> Result<Bounds, ErrorKind> {
    let minlat = try!(find_attribute::<Coordinate>("minlat", attrs).map_err(ErrorKind::BoundsMissing));
    let minlon = try!(find_attribute::<Coordinate>("minlon", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlat = try!(find_attribute::<Coordinate>("maxlat", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlon = try!(find_attribute::<Coordinate>("maxlon", attrs).map_err(ErrorKind::BoundsMissing));

    Ok(Bounds {
        minlat: minlat,
        minlon: minlon,
        maxlat: maxlat,
        maxlon: maxlon,
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
