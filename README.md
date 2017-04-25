# osm-xml

[![Build Status](https://travis-ci.org/orva/osm-xml.svg?branch=master)](https://travis-ci.org/orva/osm-xml)
[![Crates.io version](https://img.shields.io/crates/v/osm-xml.svg?maxAge=2592000)](https://crates.io/crates/osm-xml)
[![Crates.io license](https://img.shields.io/crates/l/osm-xml.svg?maxAge=2592000)](https://github.com/orva/osm-xml/blob/master/LICENSE)

Simple [osm xml v0.6][osm-xml-documentation] parser.

Structure for the parsed data follows closely how OSM documents are formed: we
have top level OSM struct containing bounds, [nodes][node-doc] array,
[ways][way-doc] array and [relations][relation-doc] array. Each parsed element
follows closely their corresponding osm-wiki entry.

References to the other elements in the document are left unresolved in parsed
form. There is API to resolve individual references to their corresponding
elements, but whole document cannot be made connected.

[Tags][tag-doc] for the element are stored in `.tags` field as array of `Tag {
key: String, val: String }`.


## Usage

Add as dependency by adding this into `Cargo.toml`:

```
[dependencies]
osm-xml = "0.5.1"
```

Then take crate into use with `extern crate osm_xml as osm;`.

Below is simple example program which digs out some statistics from given
osm-document. This includes parsing document, finding and using all the
different kind of elements and resolving references (both resolvable and
unresolvable).

```rust
extern crate osm_xml as osm;

use std::fs::File;

fn main() {
    let f = File::open("/path/to/map.osm").unwrap();
    let doc = osm::OSM::parse(f).unwrap();
    let rel_info = relation_reference_statistics(&doc);
    let way_info = way_reference_statistics(&doc);
    let poly_count = doc.ways.iter().fold(0, |acc, way| {
        if way.is_polygon() {
            return acc + 1
        }

        acc
    });

    println!("Node count {}", doc.nodes.len());
    println!("Way count {}", doc.ways.len());
    println!("Polygon count {}", poly_count);
    println!("Relation count {}", doc.relations.len());
    println!("Tag count {}", tag_count(&doc));

    println!("Way reference count: {}, invalid references: {}",  way_info.0, way_info.1);
    println!("Relation reference count: {}, resolved: {}, unresolved: {}", rel_info.0, rel_info.1, rel_info.2);
}

fn relation_reference_statistics(doc: &osm::OSM) -> (usize, usize, usize) {
    doc.relations.iter()
        .flat_map(|relation| relation.members.iter())
        .fold((0, 0, 0), |acc, member| {
            let el_ref = match *member {
                 osm::Member::Node(ref el_ref, _) => el_ref,
                 osm::Member::Way(ref el_ref, _) => el_ref,
                 osm::Member::Relation(ref el_ref, _) => el_ref,
            };

            match doc.resolve_reference(&el_ref) {
                osm::Reference::Unresolved => (acc.0 + 1, acc.1, acc.2 + 1),
                osm::Reference::Node(_)     |
                osm::Reference::Way(_)      |
                osm::Reference::Relation(_) => (acc.0 + 1, acc.1 + 1, acc.2)
            }
        })
}

fn way_reference_statistics(doc: &osm::OSM) -> (usize, usize) {
    doc.ways.iter()
        .flat_map(|way| way.nodes.iter())
        .fold((0, 0), |acc, node| {
            match doc.resolve_reference(&node) {
                osm::Reference::Node(_) => (acc.0 + 1, acc.1),
                osm::Reference::Unresolved  |
                osm::Reference::Way(_)      |
                osm::Reference::Relation(_) => (acc.0, acc.1 + 1)
            }
        })
}

fn tag_count(doc: &osm::OSM) -> usize {
    let node_tag_count = doc.nodes.iter()
        .map(|node| node.tags.len())
        .fold(0, |acc, c| acc + c);
    let way_tag_count = doc.ways.iter()
        .map(|way| way.tags.len())
        .fold(0, |acc, c| acc + c);
    let relation_tag_count = doc.relations.iter()
        .map(|relation| relation.tags.len())
        .fold(0, |acc, c| acc + c);

    node_tag_count + way_tag_count + relation_tag_count
}
```


## Features missing for 1.0

- combining OSM-structs (something simple, make it easier to update existing
  elements inside map bounds)
- writing out OSM documents
- common element attribute parsing (visible, author, changeset, etc)
- customizing parsing behaviour (short circuit on errors, optional fields, etc)
- nicer error reporting: position in the osm-document of the offending element



## Features which would be nice to have

- tag "database": would make finding elements with tags faster / saves memory on
  parsed structure as tags are just references to actual strings



## Changelog
### 0.5.1
> 2017-04-25

- Relax OSM::parse to take `Read` instead of `File`

### 0.5.0
> 2016-07-28

- Way::is_polygon

### 0.4.0
> 2016-07-22

- Initial release



## License

osm-xml is licensed under MIT-license. See more in [LICENSE][license].




[node-doc]: http://wiki.openstreetmap.org/wiki/Node
[way-doc]: http://wiki.openstreetmap.org/wiki/Way
[relation-doc]: http://wiki.openstreetmap.org/wiki/Relation
[tag-doc]: http://wiki.openstreetmap.org/wiki/Tag
[osm-xml-documentation]: http://wiki.openstreetmap.org/wiki/OSM_XML
[license]: https://github.com/orva/osm-xml/blob/master/LICENSE

