use std::collections::HashMap;
use crate::models::Produto;

pub struct Catalogo {

    //aqui, usamos um unsigned de 32bits pra indexar os produtos
    index_id: HashMap<u32, Produto>,

    index_categoria: HashMap<String, Vec<u32>>,

    index_marca: HashMap<String, Vec<u32>>
}

impl Catalogo{
    pub fn new() -> Self{
        Self{
            index_id: HashMap::new(),
            index_categoria: HashMap::new(),
            index_marca: HashMap::new()
        }
    }

    pub fn add_produto(&mut self, prod: Produto){

        let id = prod.id;
        let categoria = prod.categoria.clone();
        let marca = prod.marca.clone();

        self.index_id.insert(id, prod);
    
        self.index_categoria
            .entry(categoria)
            .or_insert(Vec::new())
            .push(id);

        self.index_marca
            .entry(marca)
            .or_insert(Vec::new())
            .push(id);
    }

    pub fn buscar_prod_id(&self, id: u32) -> Option<&Produto>{
        self.index_id.get(&id)
    }

    pub fn buscar_prod_categoria(&self, categoria: &str) -> Vec<&Produto>{
        if let Some(arr) = self.index_categoria.get(categoria){
            arr.iter().filter_map(|id| self.index_id.get(id)).collect()
        } else {
            Vec::new()
        }
    }

    pub fn buscar_prod_marca(&self, marca: &str) -> Vec<&Produto>{
        if let Some(arr) = self.index_marca.get(marca){
            arr.iter().filter_map(|id| self.index_id.get(id)).collect()
        } else {
            Vec::new()
        }
    }

}