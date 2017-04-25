use polygon;

pub type Coordinate = f64;
pub type Id = i64;
pub type Role = String;

#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    pub key: String,
    pub val: String,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Bounds {
    pub minlat: Coordinate,
    pub minlon: Coordinate,
    pub maxlat: Coordinate,
    pub maxlon: Coordinate,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: Id,
    pub lat: Coordinate,
    pub lon: Coordinate,
    pub tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Way {
    pub id: Id,
    pub tags: Vec<Tag>,
    pub nodes: Vec<UnresolvedReference>,
}

impl Way {
    pub fn is_polygon(&self) -> bool {
        polygon::is_polygon(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Relation {
    pub id: Id,
    pub members: Vec<Member>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Member {
    Node(UnresolvedReference, Role),
    Way(UnresolvedReference, Role),
    Relation(UnresolvedReference, Role),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnresolvedReference {
    Node(Id),
    Way(Id),
    Relation(Id),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Reference<'a> {
    Node(&'a Node),
    Way(&'a Way),
    Relation(&'a Relation),
    Unresolved,
}
