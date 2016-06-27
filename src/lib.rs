extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::num;

use xml::reader::{EventReader, XmlEvent};
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;

enum ErrorKind {
    BoundsMissing(AttributeError),
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
pub struct Bounds {
    minlat: f64,
    minlon: f64,
    maxlat: f64,
    maxlon: f64,
}

#[allow(dead_code)]
pub struct OSM {
    bounds: Option<Bounds>
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

fn handle_element(osm: &mut OSM, name: OwnedName, attrs: Vec<OwnedAttribute>) {
    let downcased = name.local_name.to_lowercase();

    if downcased == "bounds" {
        match parse_bounds(&attrs) {
            Ok(bounds) => osm.bounds = Some(bounds),
            Err(_) => osm.bounds = None
        }
    }
}

fn parse_bounds(attrs: &Vec<OwnedAttribute>) -> Result<Bounds, ErrorKind> {
    let minlat = try!(find_attribute::<f64>("minlat", attrs).map_err(ErrorKind::BoundsMissing));
    let minlon = try!(find_attribute::<f64>("minlon", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlat = try!(find_attribute::<f64>("maxlat", attrs).map_err(ErrorKind::BoundsMissing));
    let maxlon = try!(find_attribute::<f64>("maxlon", attrs).map_err(ErrorKind::BoundsMissing));

    Ok(Bounds {
        minlat: minlat,
        minlon: minlon,
        maxlat: maxlat,
        maxlon: maxlon,
    })
}

fn find_attribute<T: FromStr>( name: &str, attrs: &Vec<OwnedAttribute>) -> Result<T, AttributeError>
    where AttributeError: std::convert::From<<T as std::str::FromStr>::Err>
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn bounds_parsing() {
        let f = File::open("./test_data/bounds.osm").unwrap();
        let osm = OSM::parse(f).unwrap();
        let bounds = osm.bounds.unwrap();
        assert_eq!(bounds.minlat, 54.0889580);
        assert_eq!(bounds.minlon, 12.2487570);
        assert_eq!(bounds.maxlat, 54.0913900);
        assert_eq!(bounds.maxlon, 12.2524800);
    }
}
