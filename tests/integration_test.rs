use megastore::catalog::Catalog;
use megastore::generator::generate_graph;
use megastore::graph::{Graph, NodeKind};
use megastore::product::Product;
use megastore::recommender::{recommend_for_customer, recommend_for_product};
use std::collections::HashSet;

fn build_scenario() -> (Catalog, Graph) {
    let mut catalog = Catalog::new();
    let items = [
        (1, "Notebook", "Eletronicos", 3500.0),
        (2, "Mouse", "Eletronicos", 50.0),
        (3, "Teclado", "Eletronicos", 120.0),
        (4, "Livro Rust", "Livros", 90.0),
        (5, "Cadeira", "Moveis", 450.0),
    ];
    let mut graph = Graph::new();
    for (id, name, cat, price) in items {
        catalog.add(Product::new(id, name, cat, price)).unwrap();
        graph.add_product(id).unwrap();
    }
    for id in 1..=3 {
        graph.add_customer(id).unwrap();
    }
    let c = NodeKind::Customer;
    let p = NodeKind::Product;
    for (cu, pr, w) in [
        (1, 1, 5.0),
        (1, 2, 4.0),
        (2, 1, 4.0),
        (2, 3, 5.0),
        (3, 4, 5.0),
        (3, 5, 3.0),
    ] {
        graph.add_edge((c, cu), (p, pr), w).unwrap();
    }
    (catalog, graph)
}

#[test]
fn full_flow_recommends_keyboard_to_ana() {
    let (catalog, graph) = build_scenario();
    let recs = recommend_for_customer(&graph, 1, 5);
    assert_eq!(recs.len(), 1);
    let product = catalog.get(recs[0].product_id).unwrap();
    assert_eq!(product.name, "Teclado");
}

#[test]
fn customer_without_overlap_gets_no_recommendations() {
    let (_catalog, graph) = build_scenario();
    assert!(recommend_for_customer(&graph, 3, 5).is_empty());
}

#[test]
fn product_recommendation_in_scenario() {
    let (catalog, graph) = build_scenario();
    let recs = recommend_for_product(&graph, 1, 5);
    let names: HashSet<&str> = recs
        .iter()
        .map(|r| catalog.get(r.product_id).unwrap().name.as_str())
        .collect();
    assert!(names.contains("Mouse"));
    assert!(names.contains("Teclado"));
    assert!(!names.contains("Notebook"));
}

#[test]
fn synthetic_graph_has_expected_size() {
    let g = generate_graph(100, 50, 5, 4, 1);
    assert_eq!(g.node_count(), 100 + 50 + 4);
    // 100 arestas produto-categoria + 50 clientes * 5 compras
    assert_eq!(g.edge_count(), 100 + 50 * 5);
}

#[test]
fn synthetic_recommendations_are_valid() {
    let g = generate_graph(1_000, 500, 8, 20, 7);
    for cid in 1..=50u32 {
        let recs = recommend_for_customer(&g, cid, 10);
        assert!(recs.len() <= 10);

        // sem duplicatas
        let ids: HashSet<u32> = recs.iter().map(|r| r.product_id).collect();
        assert_eq!(ids.len(), recs.len());

        // nenhum produto que o cliente ja comprou
        let ci = g.node_index(NodeKind::Customer, cid).unwrap();
        let owned: HashSet<u32> = g
            .neighbors(ci)
            .iter()
            .filter(|(n, _)| g.kind_of(*n) == NodeKind::Product)
            .map(|(n, _)| g.external_id(*n))
            .collect();
        assert!(recs.iter().all(|r| !owned.contains(&r.product_id)));

        // ordenado por score decrescente
        assert!(recs.windows(2).all(|w| w[0].score >= w[1].score));
    }
}

#[test]
fn generator_is_deterministic() {
    let a = generate_graph(200, 100, 5, 5, 99);
    let b = generate_graph(200, 100, 5, 5, 99);
    assert_eq!(a.edge_count(), b.edge_count());
    assert_eq!(
        recommend_for_customer(&a, 10, 5),
        recommend_for_customer(&b, 10, 5)
    );
}