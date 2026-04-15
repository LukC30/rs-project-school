pub mod models;
pub mod search;

use models::Produto;
use search::Catalogo;

fn main() {
    println!("Iniciando o Sistema de Busca da MegaStore...\n");
    let mut catalogo = Catalogo::new();

    println!("Indexando catálogo de produtos...");
    let p1 = Produto::new(1, "Smartphone X", "TechBrand", "Eletrônicos", 2500.0);
    let p2 = Produto::new(2, "Monitor 27 Polegadas", "ViewT", "Eletrônicos", 1200.0);
    let p3 = Produto::new(3, "Camiseta Básica", "VestBrand", "Vestuário", 50.0);
    let p4 = Produto::new(4, "Cafeteira Express", "HomeBrew", "Eletrodomésticos", 350.0);

    catalogo.add_produto(p1);
    catalogo.add_produto(p2);
    catalogo.add_produto(p3);
    catalogo.add_produto(p4);
    println!("Indexação concluída com sucesso.\n");

    println!("--- Teste de Busca por ID ---");
    let id_busca = 2;
    match catalogo.buscar_prod_id(id_busca) {
        Some(prod) => println!("Encontrado: {} (R$ {})", prod.nome, prod.preco),
        None => println!("Produto com ID {} não localizado.", id_busca),
    }
    println!("\n--- Teste de Busca por Categoria ---");
    let categoria_busca = "Eletrônicos";
    let resultados = catalogo.buscar_prod_categoria(categoria_busca);

    if resultados.is_empty() {
        println!("Nenhum produto encontrado na categoria '{}'.", categoria_busca);
    } else {
        println!("{} produtos encontrados na categoria '{}':", resultados.len(), categoria_busca);
        for prod in resultados {
            println!(" - {} (Marca: {})", prod.nome, prod.marca);
        }
    }
}