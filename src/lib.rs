extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;

#[allow(dead_code)]
pub struct Bounds {
    minlat: f64,
    minlon: f64,
    maxlat: f64,
    maxlon: f64,
}

#[allow(dead_code)]
pub struct OSM {
    bounds: Bounds
}

impl OSM {
    fn empty() -> OSM {
        OSM {
            bounds: Bounds { minlat: 0.0, minlon: 0.0, maxlat: 0.0, maxlon: 0.0 }
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
        let bounds = parse_bounds(&attrs);
        osm.bounds = bounds;
    }
}

fn parse_bounds(attrs: &Vec<OwnedAttribute>) -> Bounds {
    let minlat = find_attribute::<f64>("minlat", attrs).unwrap();
    let minlon = find_attribute::<f64>("minlon", attrs).unwrap();
    let maxlat = find_attribute::<f64>("maxlat", attrs).unwrap();
    let maxlon = find_attribute::<f64>("maxlon", attrs).unwrap();

    Bounds {
        minlat: minlat,
        minlon: minlon,
        maxlat: maxlat,
        maxlon: maxlon,
    }
}

fn find_attribute<T: FromStr>(name: &str, attrs: &Vec<OwnedAttribute>) -> Option<T> {
    let attr = attrs.iter().find(|attr| attr.name.local_name == name);
    match attr {
        Some(a) => {
            let val = a.value.parse::<T>();
            match val {
                Ok(v) => Some(v),
                _ => None
            }
        },
        None => None
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
        assert_eq!(osm.bounds.minlat, 54.0889580);
        assert_eq!(osm.bounds.minlon, 12.2487570);
        assert_eq!(osm.bounds.maxlat, 54.0913900);
        assert_eq!(osm.bounds.maxlon, 12.2524800);
    }
}
