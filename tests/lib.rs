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

#[test]
fn bounds_parsing_missing_coordinate() {
    let f = File::open("./tests/test_data/bounds_missing_coord.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.bounds, None);
}

#[test]
fn bounds_parsing_invalid_coordinate() {
    let f = File::open("./tests/test_data/bounds_invalid_coord.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.bounds, None);
}

#[test]
fn no_nodes() {
    let f = File::open("./tests/test_data/bounds.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert!(osm.nodes.is_empty());
}

#[test]
fn node_existence() {
    let f = File::open("./tests/test_data/two_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes.len(), 2);
}

#[test]
fn node_ids() {
    let f = File::open("./tests/test_data/two_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes[0].id, 25496583);
    assert_eq!(osm.nodes[1].id, 25496584);
}

#[test]
fn node_coordinates() {
    let f = File::open("./tests/test_data/two_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes[0].lat, 51.5173639);
    assert_eq!(osm.nodes[0].lon, -0.140043);
    assert_eq!(osm.nodes[1].lat, 51.5173640);
    assert_eq!(osm.nodes[1].lon, -0.140041);
}

#[test]
fn skip_only_malformed_nodes() {
    let f = File::open("./tests/test_data/invalid_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes.len(), 3);

    let node = osm.nodes.iter().find(|n| n.id == 25496585);
    assert!(node.is_some());
    let node = osm.nodes.iter().find(|n| n.id == 25496586);
    assert!(node.is_some());
    let node = osm.nodes.iter().find(|n| n.id == 25496587);
    assert!(node.is_some());
}

#[test]
fn skip_malformed_node_with_child_node() {
    let f = File::open("./tests/test_data/invalid_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes.iter().find(|n| n.id == 25496583), None);
    assert_eq!(osm.nodes.iter().find(|n| n.id == 25496584), None);
}

#[test]
fn skip_malformed_node_with_child_way() {
    let f = File::open("./tests/test_data/invalid_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes.iter().find(|n| n.id == 25496588), None);
}

#[test]
fn node_tag_existence() {
    let f = File::open("./tests/test_data/two_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes[0].tags.len(), 2);
    assert_eq!(osm.nodes[1].tags.len(), 0);
}

#[test]
fn node_tag_contents() {
    let f = File::open("./tests/test_data/two_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();
    assert_eq!(osm.nodes[0].tags[0].key, "highway".to_string());
    assert_eq!(osm.nodes[0].tags[0].val, "traffic_signals".to_string());
    assert_eq!(osm.nodes[0].tags[1].key, "test_key".to_string());
    assert_eq!(osm.nodes[0].tags[1].val, "test_value".to_string());
}

#[test]
fn skip_malformed_node_tags() {
    let f = File::open("./tests/test_data/invalid_nodes.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    let node = osm.nodes.iter().find(|n| n.id == 25496587);
    assert_eq!(node.unwrap().tags.len(), 1);
}
