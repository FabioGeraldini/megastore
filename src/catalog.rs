use std::collections::HashMap;
use crate::product::Product;

#[derive(Debug, PartialEq)]
pub enum CatalogError {
    DuplicateId(u32),
}

#[derive(Default)]
pub struct Catalog {
    products: HashMap<u32, Product>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { products: HashMap::new() }
    }

    pub fn add(&mut self, product: Product) -> Result<(), CatalogError> {
        if self.products.contains_key(&product.id) {
            return Err(CatalogError::DuplicateId(product.id));
        }
        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn get(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn len(&self) -> usize {
        self.products.len()
    }

    pub fn is_empty(&self) -> bool {
        self.products.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_and_gets_product() {
        let mut c = Catalog::new();
        c.add(Product::new(1, "Notebook", "Eletronicos", 3500.0)).unwrap();
        assert_eq!(c.get(1).unwrap().name, "Notebook");
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn rejects_duplicate_id() {
        let mut c = Catalog::new();
        c.add(Product::new(1, "Notebook", "Eletronicos", 3500.0)).unwrap();
        let r = c.add(Product::new(1, "Mouse", "Eletronicos", 50.0));
        assert_eq!(r, Err(CatalogError::DuplicateId(1)));
    }

    #[test]
    fn missing_product_returns_none() {
        let c = Catalog::new();
        assert!(c.get(99).is_none());
    }
}