use std::collections::HashMap;

use super::{edge::Edge, vertex::Vertex};

#[derive(Debug)]
pub struct Graph<'a> {
    pub adjascency_list: HashMap<&'a Vertex, Vec<&'a Edge<'a>>>,
}

impl<'a> Graph<'a> {
    pub fn new(adjascency_list: HashMap<&'a Vertex, Vec<&'a Edge>>) -> Self {
        Self { adjascency_list }
    }
}
