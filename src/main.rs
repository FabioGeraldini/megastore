use megastore::catalog::Catalog;
use megastore::generator::generate_graph;
use megastore::graph::{Graph, NodeKind};
use megastore::product::Product;
use megastore::recommender::{recommend_for_customer, recommend_for_product};
use std::env;
use std::hint::black_box;
use std::time::Instant;

fn demo() {
    println!("=== DEMONSTRACAO ===");

    let mut catalog = Catalog::new();
    let items = [
        (1, "Notebook", "Eletronicos", 3500.0),
        (2, "Mouse", "Eletronicos", 50.0),
        (3, "Teclado", "Eletronicos", 120.0),
        (4, "Livro Rust", "Livros", 90.0),
        (5, "Cadeira", "Moveis", 450.0),
    ];
    for (id, name, cat, price) in items {
        catalog.add(Product::new(id, name, cat, price)).unwrap();
    }
    println!("Catalogo com {} produtos", catalog.len());

    let customers = [(1, "Ana"), (2, "Bruno"), (3, "Carla")];
    let mut graph = Graph::new();
    for (id, _) in customers {
        graph.add_customer(id).unwrap();
    }
    for (id, _, _, _) in items {
        graph.add_product(id).unwrap();
    }
    graph.add_category(1).unwrap(); // Eletronicos
    graph.add_category(2).unwrap(); // Livros

    let c = NodeKind::Customer;
    let p = NodeKind::Product;
    let cat = NodeKind::Category;

    // (cliente, produto, nota)
    let purchases = [
        (1, 1, 5.0),
        (1, 2, 4.0),
        (2, 1, 4.0),
        (2, 3, 5.0),
        (3, 4, 5.0),
        (3, 5, 3.0),
    ];
    for (cu, pr, w) in purchases {
        graph.add_edge((c, cu), (p, pr), w).unwrap();
    }
    for (pr, ca) in [(1, 1), (2, 1), (3, 1), (4, 2)] {
        graph.add_edge((p, pr), (cat, ca), 1.0).unwrap();
    }
    println!(
        "Grafo com {} vertices e {} arestas\n",
        graph.node_count(),
        graph.edge_count()
    );

    for (id, name) in customers {
        let recs = recommend_for_customer(&graph, id, 3);
        println!("Recomendacoes para {}:", name);
        if recs.is_empty() {
            println!("  (nenhuma: sem clientes com compras em comum)");
        }
        for r in recs {
            let pname = catalog
                .get(r.product_id)
                .map(|x| x.name.as_str())
                .unwrap_or("?");
            println!("  - {} (score {:.1})", pname, r.score);
        }
    }

    println!("\nQuem comprou Notebook tambem comprou:");
    for r in recommend_for_product(&graph, 1, 3) {
        let pname = catalog
            .get(r.product_id)
            .map(|x| x.name.as_str())
            .unwrap_or("?");
        println!("  - {} (score {:.1})", pname, r.score);
    }
}

fn bench() {
    println!("=== BENCHMARK (modo release recomendado) ===");
    println!("| Produtos | Clientes | Vertices | Arestas | Construcao | Rec. cliente (media) | Rec. produto (media) |");
    println!("|---|---|---|---|---|---|---|");

    for &products in &[1_000u32, 10_000, 100_000, 1_000_000] {
        let customers = products / 2;

        let t = Instant::now();
        let graph = generate_graph(products, customers, 8, 50, 42);
        let build = t.elapsed();

        let samples = 100u32;

        let t = Instant::now();
        for i in 0..samples {
            let cid = 1 + (i * (customers / samples).max(1)) % customers;
            black_box(recommend_for_customer(&graph, cid, 10));
        }
        let per_customer = t.elapsed() / samples;

        let t = Instant::now();
        for i in 0..samples {
            let pid = 1 + (i * (products / samples).max(1)) % products;
            black_box(recommend_for_product(&graph, pid, 10));
        }
        let per_product = t.elapsed() / samples;

        println!(
            "| {} | {} | {} | {} | {:.2?} | {:.2?} | {:.2?} |",
            products,
            customers,
            graph.node_count(),
            graph.edge_count(),
            build,
            per_customer,
            per_product
        );
    }
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_default();
    match arg.as_str() {
        "demo" => demo(),
        "bench" => bench(),
        _ => {
            demo();
            println!();
            bench();
        }
    }
}