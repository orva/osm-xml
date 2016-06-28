extern crate osm;

use std::fs::File;
use osm::OSM;


#[test]
fn bounds_parsing() {
    let f = File::open("./tests/test_data/bounds.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    let bounds = osm.bounds.unwrap();
    assert_eq!(bounds.minlat, 54.0889580);
    assert_eq!(bounds.minlon, 12.2487570);
    assert_eq!(bounds.maxlat, 54.0913900);
    assert_eq!(bounds.maxlon, 12.2524800);
}
