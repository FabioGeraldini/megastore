use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Customer,
    Product,
    Category,
}

#[derive(Debug, PartialEq)]
pub enum GraphError {
    DuplicateNode(NodeKind, u32),
    NodeNotFound(NodeKind, u32),
}

#[derive(Default)]
pub struct Graph {
    kinds: Vec<NodeKind>,
    external_ids: Vec<u32>,
    adjacency: Vec<Vec<(usize, f32)>>,
    index: HashMap<(NodeKind, u32), usize>,
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    fn add_node(&mut self, kind: NodeKind, id: u32) -> Result<usize, GraphError> {
        if self.index.contains_key(&(kind, id)) {
            return Err(GraphError::DuplicateNode(kind, id));
        }
        let idx = self.kinds.len();
        self.kinds.push(kind);
        self.external_ids.push(id);
        self.adjacency.push(Vec::new());
        self.index.insert((kind, id), idx);
        Ok(idx)
    }

    pub fn add_customer(&mut self, id: u32) -> Result<usize, GraphError> {
        self.add_node(NodeKind::Customer, id)
    }

    pub fn add_product(&mut self, id: u32) -> Result<usize, GraphError> {
        self.add_node(NodeKind::Product, id)
    }

    pub fn add_category(&mut self, id: u32) -> Result<usize, GraphError> {
        self.add_node(NodeKind::Category, id)
    }

    pub fn node_index(&self, kind: NodeKind, id: u32) -> Option<usize> {
        self.index.get(&(kind, id)).copied()
    }

    pub fn kind_of(&self, idx: usize) -> NodeKind {
        self.kinds[idx]
    }

    pub fn external_id(&self, idx: usize) -> u32 {
        self.external_ids[idx]
    }

    /// Aresta nao direcionada: registra nos dois sentidos.
    pub fn add_edge(
        &mut self,
        a: (NodeKind, u32),
        b: (NodeKind, u32),
        weight: f32,
    ) -> Result<(), GraphError> {
        let ia = self
            .node_index(a.0, a.1)
            .ok_or(GraphError::NodeNotFound(a.0, a.1))?;
        let ib = self
            .node_index(b.0, b.1)
            .ok_or(GraphError::NodeNotFound(b.0, b.1))?;
        self.adjacency[ia].push((ib, weight));
        self.adjacency[ib].push((ia, weight));
        Ok(())
    }

    pub fn neighbors(&self, idx: usize) -> &[(usize, f32)] {
        &self.adjacency[idx]
    }

    pub fn node_count(&self) -> usize {
        self.kinds.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency.iter().map(|v| v.len()).sum::<usize>() / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_nodes_and_rejects_duplicates() {
        let mut g = Graph::new();
        g.add_customer(1).unwrap();
        assert_eq!(
            g.add_customer(1),
            Err(GraphError::DuplicateNode(NodeKind::Customer, 1))
        );
        // mesmo ID em tipo diferente e permitido
        assert!(g.add_product(1).is_ok());
        assert_eq!(g.node_count(), 2);
    }

    #[test]
    fn edge_is_bidirectional() {
        let mut g = Graph::new();
        let c = g.add_customer(1).unwrap();
        let p = g.add_product(10).unwrap();
        g.add_edge((NodeKind::Customer, 1), (NodeKind::Product, 10), 5.0)
            .unwrap();
        assert_eq!(g.neighbors(c), &[(p, 5.0)]);
        assert_eq!(g.neighbors(p), &[(c, 5.0)]);
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn edge_with_missing_node_fails() {
        let mut g = Graph::new();
        g.add_customer(1).unwrap();
        let r = g.add_edge((NodeKind::Customer, 1), (NodeKind::Product, 99), 1.0);
        assert_eq!(r, Err(GraphError::NodeNotFound(NodeKind::Product, 99)));
    }

    #[test]
    fn lookup_by_kind_and_id() {
        let mut g = Graph::new();
        let p = g.add_product(7).unwrap();
        assert_eq!(g.node_index(NodeKind::Product, 7), Some(p));
        assert_eq!(g.node_index(NodeKind::Customer, 7), None);
        assert_eq!(g.kind_of(p), NodeKind::Product);
        assert_eq!(g.external_id(p), 7);
    }
}