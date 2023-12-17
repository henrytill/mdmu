use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

use time::Date;
use url::Url;

/// An [`Id`] is a unique identifier for an [`Entity`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(usize);

impl Id {
    const fn new(id: usize) -> Self {
        Self(id)
    }
}

impl From<Id> for usize {
    fn from(handle: Id) -> Self {
        handle.0
    }
}

/// A [`Name`] describes an [`Entity`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub const fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for Name {
    fn from(name: String) -> Self {
        Self(name)
    }
}

/// A [`Label`] is a label that can be attached to an [`Entity`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(String);

impl Label {
    pub const fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Hash for Label {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<&str> for Label {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for Label {
    fn from(name: String) -> Self {
        Self(name)
    }
}

/// An [`Entity`] is a page in the collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    url: Url,
    created_at: Date,
    updated_at: HashSet<Date>,
    names: HashSet<Name>,
    labels: HashSet<Label>,
}

impl Entity {
    pub fn new(
        url: Url,
        created_at: Date,
        maybe_name: Option<Name>,
        labels: HashSet<Label>,
    ) -> Self {
        let updated_at = HashSet::new();
        let names = maybe_name.into_iter().collect();
        Self {
            url,
            created_at,
            updated_at,
            names,
            labels,
        }
    }

    pub fn update(
        &mut self,
        updated_at: Date,
        names: HashSet<Name>,
        labels: HashSet<Label>,
    ) -> &mut Self {
        if updated_at < self.created_at {
            self.updated_at.insert(self.created_at);
            self.created_at = updated_at;
        } else {
            self.updated_at.insert(updated_at);
        }
        self.names.extend(names);
        self.labels.extend(labels);
        self
    }

    pub fn merge(&mut self, other: Self) -> &mut Self {
        self.update(other.created_at, other.names, other.labels)
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn created_at(&self) -> &Date {
        &self.created_at
    }

    pub fn updated_at(&self) -> &HashSet<Date> {
        &self.updated_at
    }

    pub fn names(&self) -> &HashSet<Name> {
        &self.names
    }

    pub fn labels(&self) -> &HashSet<Label> {
        &self.labels
    }
}

pub type Edges = Vec<Id>;

/// A collection of entities.
///
/// This is a graph structure where a nodes are represented by a vector of entities and edges are
/// represented by an adjacency list.
#[derive(Debug)]
pub struct Collection {
    nodes: Vec<Entity>,
    edges: Vec<Edges>,
    urls: HashMap<Url, Id>,
}

impl Index<Id> for Vec<Entity> {
    type Output = Entity;

    fn index(&self, id: Id) -> &Self::Output {
        &self[id.0]
    }
}

impl IndexMut<Id> for Vec<Entity> {
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        &mut self[id.0]
    }
}

impl Index<Id> for Vec<Edges> {
    type Output = Edges;

    fn index(&self, id: Id) -> &Self::Output {
        &self[id.0]
    }
}

impl IndexMut<Id> for Vec<Edges> {
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        &mut self[id.0]
    }
}

impl Collection {
    pub fn new() -> Self {
        let nodes = Vec::new();
        let edges = Vec::new();
        let urls = HashMap::new();
        Self { nodes, edges, urls }
    }

    pub fn len(&self) -> usize {
        let len = self.nodes.len();
        assert_eq!(len, self.edges.len());
        len
    }

    pub fn is_empty(&self) -> bool {
        let is_empty = self.nodes.is_empty();
        assert!(self.edges.is_empty());
        is_empty
    }

    pub fn contains(&self, url: &Url) -> bool {
        self.urls.contains_key(url)
    }

    pub fn id(&self, url: &Url) -> Option<Id> {
        self.urls.get(url).copied()
    }

    pub fn add(&mut self, entity: Entity) -> Id {
        assert_eq!(self.nodes.len(), self.edges.len());
        let id = Id::new(self.len());
        self.nodes.push(entity);
        self.edges.push(Vec::new());
        let url = self.nodes[id].url().to_owned();
        self.urls.insert(url, id);
        id
    }

    pub fn merge(&mut self, other: Entity) -> Id {
        let id = if let Some(id) = self.id(other.url()) {
            id
        } else {
            return self.add(other);
        };
        let entity = &mut self.nodes[id];
        entity.merge(other);
        id
    }

    pub fn add_edge(&mut self, from: Id, to: Id) {
        let from_edges = &mut self.edges[from];
        if from_edges.contains(&to) {
            return;
        }
        from_edges.push(to);
    }

    pub fn entity(&self, id: Id) -> &Entity {
        &self.nodes[id]
    }

    pub fn entity_mut(&mut self, id: Id) -> &mut Entity {
        &mut self.nodes[id]
    }

    pub fn edges(&self, id: Id) -> &[Id] {
        &self.edges[id]
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self::new()
    }
}
