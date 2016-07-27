pub type Coordinate = f64;
pub type Id = i64;
pub type Role = String;

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub key: String,
    pub val: String,
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
    pub tags: Vec<Tag>,
}

#[derive(Debug, PartialEq)]
pub struct Way {
    pub id: Id,
    pub tags: Vec<Tag>,
    pub nodes: Vec<UnresolvedReference>,
}

#[derive(Debug, PartialEq)]
pub struct Relation {
    pub id: Id,
    pub members: Vec<Member>,
    pub tags: Vec<Tag>,
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
    Relation(Id),
}

#[derive(Debug, PartialEq)]
pub enum Reference<'a> {
    Node(&'a Node),
    Way(&'a Way),
    Relation(&'a Relation),
    Unresolved,
}
