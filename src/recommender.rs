use crate::graph::{Graph, NodeKind};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub struct Recommendation {
    pub product_id: u32,
    pub score: f32,
}

// Wrapper para ordenar f32 no BinaryHeap (min-heap pelo score).
#[derive(Debug, PartialEq)]
struct HeapItem {
    score: f32,
    product_id: u32,
}

impl Eq for HeapItem {}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // invertido: o menor score fica no topo (min-heap)
        other
            .score
            .partial_cmp(&self.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| other.product_id.cmp(&self.product_id))
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Mantem apenas os k maiores scores usando um min-heap de tamanho k.
fn top_k(scores: HashMap<u32, f32>, k: usize) -> Vec<Recommendation> {
    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::with_capacity(k + 1);
    for (product_id, score) in scores {
        heap.push(HeapItem { score, product_id });
        if heap.len() > k {
            heap.pop();
        }
    }
    let mut result: Vec<Recommendation> = heap
        .into_iter()
        .map(|h| Recommendation {
            product_id: h.product_id,
            score: h.score,
        })
        .collect();
    result.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.product_id.cmp(&b.product_id))
    });
    result
}

/// Recomenda produtos para um cliente via BFS limitado (profundidade 3).
pub fn recommend_for_customer(graph: &Graph, customer_id: u32, k: usize) -> Vec<Recommendation> {
    let start = match graph.node_index(NodeKind::Customer, customer_id) {
        Some(i) => i,
        None => return Vec::new(),
    };

    // produtos que o cliente ja tem (nao podem ser recomendados)
    let owned: HashSet<usize> = graph
        .neighbors(start)
        .iter()
        .filter(|(n, _)| graph.kind_of(*n) == NodeKind::Product)
        .map(|(n, _)| *n)
        .collect();

    // (no, profundidade, peso acumulado do caminho)
    let mut queue: VecDeque<(usize, u8, f32)> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut scores: HashMap<usize, f32> = HashMap::new();

    visited.insert(start);
    queue.push_back((start, 0, 1.0));

    while let Some((node, depth, acc)) = queue.pop_front() {
        if depth == 3 {
            continue;
        }
        for &(next, w) in graph.neighbors(node) {
            let kind = graph.kind_of(next);
            if kind == NodeKind::Category {
                continue;
            }
            let new_acc = acc * w;

            if depth == 2 {
                // nivel dos candidatos: so produtos que o cliente nao possui
                if kind == NodeKind::Product && !owned.contains(&next) {
                    *scores.entry(next).or_insert(0.0) += new_acc;
                }
                continue;
            }

            // niveis 0 e 1: alterna cliente -> produto -> cliente
            let expected = if depth == 0 || depth == 2 {
                NodeKind::Product
            } else {
                NodeKind::Customer
            };
            if kind == expected && visited.insert(next) {
                queue.push_back((next, depth + 1, new_acc));
            }
        }
    }

    let by_id: HashMap<u32, f32> = scores
        .into_iter()
        .map(|(idx, s)| (graph.external_id(idx), s))
        .collect();
    top_k(by_id, k)
}

/// Recomenda "quem comprou isto tambem comprou": produto -> clientes -> outros produtos.
pub fn recommend_for_product(graph: &Graph, product_id: u32, k: usize) -> Vec<Recommendation> {
    let start = match graph.node_index(NodeKind::Product, product_id) {
        Some(i) => i,
        None => return Vec::new(),
    };

    let mut scores: HashMap<u32, f32> = HashMap::new();
    let mut seen_customers: HashSet<usize> = HashSet::new();

    for &(customer, w1) in graph.neighbors(start) {
        if graph.kind_of(customer) != NodeKind::Customer || !seen_customers.insert(customer) {
            continue;
        }
        for &(other, w2) in graph.neighbors(customer) {
            if graph.kind_of(other) == NodeKind::Product && other != start {
                *scores.entry(graph.external_id(other)).or_insert(0.0) += w1 * w2;
            }
        }
    }
    top_k(scores, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> Graph {
        let mut g = Graph::new();
        for c in 1..=3 {
            g.add_customer(c).unwrap();
        }
        for p in 1..=5 {
            g.add_product(p).unwrap();
        }
        let c = NodeKind::Customer;
        let p = NodeKind::Product;
        // 1=Ana 2=Bruno 3=Carla | 1=Notebook 2=Mouse 3=Teclado 4=Livro 5=Cadeira
        g.add_edge((c, 1), (p, 1), 5.0).unwrap();
        g.add_edge((c, 1), (p, 2), 4.0).unwrap();
        g.add_edge((c, 2), (p, 1), 4.0).unwrap();
        g.add_edge((c, 2), (p, 3), 5.0).unwrap();
        g.add_edge((c, 3), (p, 4), 5.0).unwrap();
        g.add_edge((c, 3), (p, 5), 3.0).unwrap();
        g
    }

    #[test]
    fn recommends_keyboard_to_ana() {
        let g = sample_graph();
        let r = recommend_for_customer(&g, 1, 5);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].product_id, 3);
    }

    #[test]
    fn does_not_recommend_owned_products() {
        let g = sample_graph();
        let r = recommend_for_customer(&g, 1, 10);
        assert!(r.iter().all(|x| x.product_id != 1 && x.product_id != 2));
    }

    #[test]
    fn no_duplicate_recommendations() {
        let g = sample_graph();
        let r = recommend_for_customer(&g, 2, 10);
        let ids: HashSet<u32> = r.iter().map(|x| x.product_id).collect();
        assert_eq!(ids.len(), r.len());
    }

    #[test]
    fn unknown_customer_returns_empty() {
        let g = sample_graph();
        assert!(recommend_for_customer(&g, 99, 5).is_empty());
    }

    #[test]
    fn respects_top_k_limit() {
        let g = sample_graph();
        let r = recommend_for_customer(&g, 2, 1);
        assert!(r.len() <= 1);
    }

    #[test]
    fn recommends_from_product() {
        let g = sample_graph();
        let r = recommend_for_product(&g, 1, 5);
        let ids: Vec<u32> = r.iter().map(|x| x.product_id).collect();
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
        assert!(!ids.contains(&1));
    }
}