use sistema_megastore::models::Produto;
use sistema_megastore::search::Catalogo;

// A macro #[test] indica ao compilador que esta função deve ser executada pelo comando `cargo test`
#[test]
fn deve_adicionar_e_buscar_produto_por_id() {
    let mut catalogo = Catalogo::new();
    let prod = Produto::new(1, "Notebook", "TechBrand", "Eletrônicos", 3500.00);
    
    catalogo.add_produto(prod);
    
    // assert_eq! verifica se o valor da esquerda é igual ao da direita. 
    // Se não for, o teste falha (panic) e o Cargo avisa.
    let resultado = catalogo.buscar_prod_id(1);
    assert_eq!(resultado.is_some(), true);
    assert_eq!(resultado.unwrap().nome, "Notebook");
}

#[test]
fn deve_buscar_produtos_por_categoria() {
    let mut catalogo = Catalogo::new();
    let prod1 = Produto::new(1, "Mouse", "TechBrand", "Eletrônicos", 150.00);
    let prod2 = Produto::new(2, "Teclado", "TechBrand", "Eletrônicos", 250.00);
    let prod3 = Produto::new(3, "Camiseta", "VestBrand", "Vestuário", 80.00);
    
    catalogo.add_produto(prod1);
    catalogo.add_produto(prod2);
    catalogo.add_produto(prod3);
    
    let eletronicos = catalogo.buscar_prod_categoria("Eletrônicos");
    let vestuario = catalogo.buscar_prod_categoria("Vestuário");
    let inexistente = catalogo.buscar_prod_categoria("Alimentos");
    
    // Validamos se os tamanhos dos vetores correspondem à quantidade inserida
    assert_eq!(eletronicos.len(), 2);
    assert_eq!(vestuario.len(), 1);
    assert_eq!(inexistente.len(), 0);
}