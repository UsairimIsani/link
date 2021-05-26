use std::collections::HashMap;
use std::hash::Hash;

pub struct Graph<VertexId, E = (), V = ()> {
    vertices: HashMap<VertexId, V>,
    adjacency: HashMap<VertexId, Vec<(VertexId, E)>>,
}
impl<VertexId, E, V> Graph<VertexId, E, V>
where
    VertexId: Eq + Hash,
    V: Hash,
{
    /// Create snew Graph
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    /// Insert a vertex
    pub fn insert_vertex(&mut self, vid: VertexId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    /// Insert an edge between vertices.
    pub fn insert_edge(&mut self, from: VertexId, to: VertexId, edge: E) {
        let to_from = self.adjacency.entry(from).or_default();
        to_from.push((to, edge));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_graph() {
        use super::*;
        let g: Graph<u32, u32, u32> = Graph::new();
    }
}
