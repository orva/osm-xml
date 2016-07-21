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

#[test]
fn way_existence() {
    let f = File::open("./tests/test_data/way.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    assert_eq!(osm.ways.len(), 2);
    assert_eq!(osm.ways[0].id, 4253174);
    assert_eq!(osm.ways[1].id, 4253123);
}

#[test]
fn way_tags() {
    let f = File::open("./tests/test_data/way.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    assert_eq!(osm.ways[0].tags.len(), 11);
    assert_eq!(osm.ways[0].tags[0].key, "highway".to_string());
    assert_eq!(osm.ways[0].tags[0].val, "residential".to_string());
    assert_eq!(osm.ways[0].tags[1].key, "lanes".to_string());
    assert_eq!(osm.ways[0].tags[1].val, "1".to_string());
    assert_eq!(osm.ways[0].tags[2].key, "lit".to_string());
    assert_eq!(osm.ways[0].tags[2].val, "yes".to_string());
    assert_eq!(osm.ways[0].tags[3].key, "maxspeed".to_string());
    assert_eq!(osm.ways[0].tags[3].val, "30".to_string());
    assert_eq!(osm.ways[0].tags[4].key, "name".to_string());
    assert_eq!(osm.ways[0].tags[4].val, "Maurinkatu".to_string());
    assert_eq!(osm.ways[0].tags[5].key, "name:fi".to_string());
    assert_eq!(osm.ways[0].tags[5].val, "Maurinkatu".to_string());
    assert_eq!(osm.ways[0].tags[6].key, "name:sv".to_string());
    assert_eq!(osm.ways[0].tags[6].val, "Mauritzgatan".to_string());
    assert_eq!(osm.ways[0].tags[7].key, "old_name:fi".to_string());
    assert_eq!(osm.ways[0].tags[7].val, "Mauritsinkatu".to_string());
    assert_eq!(osm.ways[0].tags[8].key, "snowplowing".to_string());
    assert_eq!(osm.ways[0].tags[8].val, "yes".to_string());
    assert_eq!(osm.ways[0].tags[9].key, "start_date".to_string());
    assert_eq!(osm.ways[0].tags[9].val, "before 1815".to_string());
    assert_eq!(osm.ways[0].tags[10].key, "surface".to_string());
    assert_eq!(osm.ways[0].tags[10].val, "paved".to_string());
}

#[test]
fn way_node_references() {
    let f = File::open("./tests/test_data/way.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    let nodes = &osm.ways[0].nodes;
    assert_eq!(nodes.len(), 7);

    match nodes[0] {
        osm::UnresolvedReference::Node(id) => assert_eq!(id, 1375815878),
        _ => assert!(false, "Way reference was not reference to Node!")
    }
    match nodes[1] {
        osm::UnresolvedReference::Node(id) => assert_eq!(id, 391448656),
        _ => assert!(false, "Way reference was not reference to Node!")
    }
    match nodes[2] {
        osm::UnresolvedReference::Node(id) => assert_eq!(id, 340886677),
        _ => assert!(false, "Way reference was not reference to Node!")
    }
    match nodes[3] {
        osm::UnresolvedReference::Node(id) => assert_eq!(id, 1651393269),
        _ => assert!(false, "Way reference was not reference to Node!")
    }
    match nodes[4] {
        osm::UnresolvedReference::Node(id) => assert_eq!(id, 471408613),
        _ => assert!(false, "Way reference was not reference to Node!")
    }
    match nodes[5] {
        osm::UnresolvedReference::Node(id) => assert_eq!(id, 25470395),
        _ => assert!(false, "Way reference was not reference to Node!")
    }
    match nodes[6] {
        osm::UnresolvedReference::Node(id) => assert_eq!(id, 1376857625),
        _ => assert!(false, "Way reference was not reference to Node!")
    }
}

#[test]
fn relation_existence() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    assert_eq!(osm.relations.len(), 6);
    assert_eq!(osm.relations[0].id, 77994);
    assert_eq!(osm.relations[1].id, 1688359);
    assert_eq!(osm.relations[2].id, 375952);
    assert_eq!(osm.relations[3].id, 375951);
    assert_eq!(osm.relations[4].id, 155054);
    assert_eq!(osm.relations[5].id, 987654);
}

#[test]
fn relation_tags() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    let tags = &osm.relations[0].tags;
    assert_eq!(tags[0].key, "type".to_string());
    assert_eq!(tags[0].val, "surveillance".to_string());
}

#[test]
fn relation_node_members() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    match osm.relations[0].members[0] {
        osm::Member::Node(ref ref_id, ref role) => {
            match *ref_id {
                osm::UnresolvedReference::Node(id) => assert_eq!(id, 345579224),
                _ => assert!(false, "Member reference was wrong type!")
            }
            assert_eq!(*role, "camera".to_string());
        },
        _ => assert!(false, "Member was not expected Member-variant")
    }

    match osm.relations[0].members[1] {
        osm::Member::Node(ref ref_id, ref role) => {
            match *ref_id {
                osm::UnresolvedReference::Node(id) => assert_eq!(id, 345579225),
                _ => assert!(false, "Member reference was wrong type!")
            }
            assert_eq!(*role, "visible".to_string());
        },
        _ => assert!(false, "Member was not expected Member-variant")
    }
}

#[test]
fn relation_way_members() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    match osm.relations[1].members[0] {
        osm::Member::Way(ref ref_id, ref role) => {
            match *ref_id {
                osm::UnresolvedReference::Way(id) => assert_eq!(id, 123365172),
                _ => assert!(false, "Member reference was wrong type!")
            }
            assert_eq!(*role, "outer".to_string());
        },
        _ => assert!(false, "Member was not expected Member-variant")
    }

    match osm.relations[1].members[1] {
        osm::Member::Way(ref ref_id, ref role) => {
            match *ref_id {
                osm::UnresolvedReference::Way(id) => assert_eq!(id, 22147620),
                _ => assert!(false, "Member reference was wrong type!")
            }
            assert_eq!(*role, "inner".to_string());
        },
        _ => assert!(false, "Member was not expected Member-variant")
    }
}

#[test]
fn way_reference_resolving() {
    let f = File::open("./tests/test_data/way.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    for node_ref in osm.ways[0].nodes.iter() {
        match osm.resolve_reference(node_ref) {
            osm::Reference::Node(_) => continue,
            osm::Reference::Unresolved => assert!(false, "Resolvable way node was not resolved!"),
            _ => assert!(false, "Valid way references are always Nodes!")
        }
    }
}

#[test]
fn way_invalid_reference_resolving() {
    let f = File::open("./tests/test_data/way.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    match osm.resolve_reference(&osm.ways[1].nodes[0]) {
        osm::Reference::Node(node) => assert_eq!(*node, osm.nodes[0]),
        osm::Reference::Unresolved => assert!(false, "Resolvable way node was not resolved!"),
        _ => assert!(false, "Valid way references are always Nodes!")
    }

    match osm.resolve_reference(&osm.ways[1].nodes[1]) {
        osm::Reference::Unresolved => (),
        _ => assert!(false, "Unresolvable Node reference was resolved!"),
    }
}

#[test]
fn relation_node_reference_resolving() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    match osm.relations[0].members[0] {
        osm::Member::Node(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Node(node) => assert_eq!(*node, osm.nodes[0]),
            _ => assert!(false, "Resolvable Relation member was not resolved!")
        },
        _ => assert!(false, "Member should have been Node!")
    }

    match osm.relations[0].members[1] {
        osm::Member::Node(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Node(node) => assert_eq!(*node, osm.nodes[1]),
            _ => assert!(false, "Resolvable Relation member was not resolved!")
        },
        _ => assert!(false, "Member should have been Node!")
    }
}

#[test]
fn relation_way_reference_resolving() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    match osm.relations[1].members[0] {
        osm::Member::Way(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Way(way) => assert_eq!(*way, osm.ways[0]),
            _ => assert!(false, "Resolvable Relation member was not resolved!")
        },
        _ => assert!(false, "Member should have been Way!")
    }

    match osm.relations[1].members[1] {
        osm::Member::Way(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Way(way) => assert_eq!(*way, osm.ways[1]),
            _ => assert!(false, "Resolvable Relation member was not resolved!")
        },
        _ => assert!(false, "Member should have been Way!")
    }
}

#[test]
fn relation_relation_reference_resolving() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    match osm.relations[4].members[0] {
        osm::Member::Relation(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Relation(rel) => assert_eq!(*rel, osm.relations[2]),
            _ => assert!(false, "Resolvable Relation member was not resolved!")
        },
        _ => assert!(false, "Member should have been Relation!")
    }

    match osm.relations[4].members[1] {
        osm::Member::Relation(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Relation(rel) => assert_eq!(*rel, osm.relations[3]),
            _ => assert!(false, "Resolvable Relation member was not resolved!")
        },
        _ => assert!(false, "Member should have been Relation!")
    }
}

#[test]
fn relation_with_unresolvable_node() {
    let f = File::open("./tests/test_data/relations.osm").unwrap();
    let osm = OSM::parse(f).unwrap();

    match osm.relations[5].members[0] {
        osm::Member::Node(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Unresolved => assert!(true),
            _ => assert!(false, "Unresolvable reference was resolved")
        },
        _ => assert!(false, "Member should have been Node!")
    }

    match osm.relations[5].members[1] {
        osm::Member::Way(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Unresolved => assert!(true),
            _ => assert!(false, "Unresolvable reference was resolved")
        },
        _ => assert!(false, "Member should have been Way!")
    }

    match osm.relations[5].members[2] {
        osm::Member::Relation(ref mref, _) => match osm.resolve_reference(mref) {
            osm::Reference::Unresolved => assert!(true),
            _ => assert!(false, "Unresolvable reference was resolved")
        },
        _ => assert!(false, "Member should have been Relation!")
    }
}

