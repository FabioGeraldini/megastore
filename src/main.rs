mod catalog;
mod graph;
mod product;

use catalog::Catalog;
use product::Product;

fn main() {
    let mut catalog = Catalog::new();
    catalog.add(Product::new(1, "Notebook", "Eletronicos", 3500.0)).unwrap();
    catalog.add(Product::new(2, "Mouse", "Eletronicos", 50.0)).unwrap();

    match catalog.get(1) {
        Some(p) => println!("Produto encontrado: {:?}", p),
        None => println!("Produto nao encontrado"),
    }
    println!("Total de produtos: {}", catalog.len());
}