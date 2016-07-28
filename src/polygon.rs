use elements::{Way, Tag};

struct Rule {
    key: &'static str,
    polygon: RuleType,
    values: [&'static str; 6],
}

enum RuleType {
    All,
    Blacklist,
    Whitelist,
}

static RULES: [Rule; 26] =
    [Rule {
         key: "building",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "highway",
         polygon: RuleType::Whitelist,
         values: ["services", "rest_area", "escape", "elevator", "", ""],
     },
     Rule {
         key: "natural",
         polygon: RuleType::Blacklist,
         values: ["coastline", "cliff", "ridge", "arete", "tree_row", ""],
     },
     Rule {
         key: "landuse",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "waterway",
         polygon: RuleType::Whitelist,
         values: ["riverbank", "dock", "boatyard", "dam", "", ""],
     },
     Rule {
         key: "amenity",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "leisure",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "barrier",
         polygon: RuleType::Whitelist,
         values: ["city_wall", "ditch", "hedge", "retaining_wall", "wall", "spikes"],
     },
     Rule {
         key: "railway",
         polygon: RuleType::Whitelist,
         values: ["station", "turntable", "roundhouse", "platform", "", ""],
     },
     Rule {
         key: "area",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "boundary",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "man_made",
         polygon: RuleType::Blacklist,
         values: ["cutline", "embankment", "pipeline", "", "", ""],
     },
     Rule {
         key: "power",
         polygon: RuleType::Whitelist,
         values: ["plant", "substation", "generator", "transformer", "", ""],
     },
     Rule {
         key: "place",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "shop",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "aeroway",
         polygon: RuleType::Blacklist,
         values: ["taxiway", "", "", "", "", ""],
     },
     Rule {
         key: "tourism",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "historic",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "public_transport",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "office",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "building:part",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "military",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "ruins",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "area:highway",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "craft",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "golf",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     }];

pub fn is_polygon(way: &Way) -> bool {
    if is_closed_loop(way) {
        return true;
    }

    way.tags.iter().any(|tag| {
        RULES.iter().any(|rule| {
            rule.key == tag.key && tag.val != "no" && has_matching_rule_value(rule, tag)
        })
    })
}

fn is_closed_loop(way: &Way) -> bool {
    let first = way.nodes.first();
    let last = way.nodes.last();

    if let None = first.and(last) {
        return false;
    }

    first.unwrap() == last.unwrap()
}

fn has_matching_rule_value(rule: &Rule, tag: &Tag) -> bool {
    match rule.polygon {
        RuleType::All => true,
        RuleType::Whitelist => rule.values.iter().any(|val| *val != "" && *val == tag.val),
        RuleType::Blacklist => !rule.values.iter().any(|val| *val == tag.val),
    }
}





#[cfg(test)]
mod test {
    use super::*;
    use elements::{Way, Tag, UnresolvedReference};

    #[test]
    fn tagless_and_nonloop_is_not_polygon() {
        let way = Way {
            id: 1234567,
            tags: Vec::new(),
            nodes: Vec::new(),
        };

        assert!(!is_polygon(&way));
    }

    #[test]
    fn closed_loop_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: Vec::new(),
            nodes: vec![
                UnresolvedReference::Node(1),
                UnresolvedReference::Node(2),
                UnresolvedReference::Node(3),
                UnresolvedReference::Node(26),
                UnresolvedReference::Node(1),
                ],
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn detect_ruletype_all_with_tag_val() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("building"),
                           val: String::from("this_is_not_valid"),
                       }],
            nodes: Vec::new(),
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn detect_ruletype_all_without_tag_val() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("building"),
                           val: String::from(""),
                       }],
            nodes: Vec::new(),
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn whitelist_val_included_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("highway"),
                           val: String::from("escape"),
                       }],
            nodes: Vec::new(),
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn whitelist_val_not_included_is_not_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("highway"),
                           val: String::from("footway"),
                       }],
            nodes: Vec::new(),
        };

        assert!(!is_polygon(&way));
    }

    #[test]
    fn whitelist_with_empty_val_is_not_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("highway"),
                           val: String::from(""),
                       }],
            nodes: Vec::new(),
        };

        assert!(!is_polygon(&way));
    }

    #[test]
    fn whitelist_with_matching_and_nonmatching_tags_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![
                Tag { key: String::from("highway"), val: String::from("footway") },
                Tag { key: String::from("highway"), val: String::from("escape") },
                ],
            nodes: Vec::new(),
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn nonloop_and_whitelist_match_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("highway"),
                           val: String::from("escape"),
                       }],
            nodes: vec![
                UnresolvedReference::Node(1),
                UnresolvedReference::Node(2),
                UnresolvedReference::Node(3),
                ],
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn blacklist_val_included_is_not_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("natural"),
                           val: String::from("cliff"),
                       }],
            nodes: Vec::new(),
        };

        assert!(!is_polygon(&way));
    }

    #[test]
    fn blacklist_val_not_included_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("natural"),
                           val: String::from("tree"),
                       }],
            nodes: Vec::new(),
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn blacklist_with_empty_val_is_not_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("natural"),
                           val: String::from(""),
                       }],
            nodes: Vec::new(),
        };

        assert!(!is_polygon(&way));
    }

    #[test]
    fn blacklist_with_matching_and_nonmatching_tags_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![
                Tag { key: String::from("natural"), val: String::from("cliff") },
                Tag { key: String::from("natural"), val: String::from("tree") },
                ],
            nodes: Vec::new(),
        };

        assert!(is_polygon(&way));
    }

    #[test]
    fn nonloop_and_blacklist_cleared_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: vec![Tag {
                           key: String::from("natural"),
                           val: String::from("tree"),
                       }],
            nodes: vec![
                UnresolvedReference::Node(1),
                UnresolvedReference::Node(2),
                UnresolvedReference::Node(3),
                ],
        };

        assert!(is_polygon(&way));
    }


    #[test]
    fn rules_with_no_value_are_not_polygons() {
        let keys = vec![
            "building",
            "highway",
            "natural",
            "landuse",
            "waterway",
            "amenity",
            "leisure",
            "barrier",
            "railway",
            "area",
            "boundary",
            "man_made",
            "power",
            "place",
            "shop",
            "aeroway",
            "tourism",
            "historic",
            "public_transport",
            "office",
            "building:part",
            "military",
            "ruins",
            "area:highway",
            "craft",
            "golf",
            ];

        let ways = keys.iter().map(|key| {
            return Way {
                id: 1234567,
                tags: vec![ Tag { key: String::from(*key), val: String::from("no") }, ],
                nodes: Vec::new(),
            };
        });

        for way in ways {
            assert!(!is_polygon(&way));
        }
    }
}
