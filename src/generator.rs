use crate::graph::{Graph, NodeKind};

/// Gerador pseudoaleatorio simples (LCG), sem dependencias externas.
/// Com a mesma semente, gera sempre os mesmos dados (testes reproduziveis).
pub struct Lcg(u64);

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Lcg(seed)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) as u32
    }

    /// Numero em [0, n)
    pub fn range(&mut self, n: u32) -> u32 {
        self.next_u32() % n
    }
}

/// Gera um grafo sintetico:
/// - `products` produtos, `customers` clientes, `categories` categorias
/// - cada produto liga a uma categoria (peso 1.0)
/// - cada cliente compra `purchases` produtos distintos, com nota de 1 a 5
pub fn generate_graph(
    products: u32,
    customers: u32,
    purchases: u32,
    categories: u32,
    seed: u64,
) -> Graph {
    let categories = categories.max(1);
    let mut g = Graph::new();
    let mut rng = Lcg::new(seed);

    for p in 1..=products {
        g.add_product(p).unwrap();
    }
    for c in 1..=customers {
        g.add_customer(c).unwrap();
    }
    for k in 1..=categories {
        g.add_category(k).unwrap();
    }

    for p in 1..=products {
        let cat = (p % categories) + 1;
        g.add_edge((NodeKind::Product, p), (NodeKind::Category, cat), 1.0)
            .unwrap();
    }

    let n = purchases.min(products) as usize;
    let mut picked: Vec<u32> = Vec::with_capacity(n);
    for c in 1..=customers {
        picked.clear();
        while picked.len() < n {
            let p = rng.range(products) + 1;
            if !picked.contains(&p) {
                picked.push(p);
                let rating = (rng.range(5) + 1) as f32;
                g.add_edge((NodeKind::Customer, c), (NodeKind::Product, p), rating)
                    .unwrap();
            }
        }
    }
    g
}