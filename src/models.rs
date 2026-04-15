#[derive(Debug, Clone)]
pub struct Produto{
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
    pub preco: f64,
}

//a impl serve para unir metodos na struct
impl Produto {
    // new é o construtor da struct
    pub fn new(id: u32, nome: &str, marca: &str, categoria: &str, preco: f64) -> Self {
        return Self{
            id,
            nome: nome.to_string(),
            marca: marca.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }
}