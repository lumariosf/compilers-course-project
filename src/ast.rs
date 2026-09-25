#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Grafo(Grafo),
    Consulta(ExpressaoConsulta),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grafo {
    pub nome: String,
    pub nodes: Vec<String>,
    pub edges: Vec<Aresta>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemGrafoParcial {
    Node(Vec<String>), 
    Edge(Vec<Aresta>), 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Aresta {
    pub origem: String,
    pub destino: String,
    pub tipo: TipoAresta,
    pub peso: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TipoAresta {
    Direcionada,   // ->
    NaoDirecionada, // --
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressaoConsulta {
    pub grafo: String,
    pub chamadas: Vec<Chamada>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Chamada {
    ShortDist { origem: String, destino: String },
    Filter { filtro: Filtro },
    Merge { grafo_alvo: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filtro {
    pub variavel: String,
    pub fonte: String,
    pub condicao: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MergeArgs {
    pub origem: (String, String), // (grafo, node)
    pub destino: (String, String),
    pub tipo: TipoAresta,
    pub peso: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Numero(f64),
    Texto(String),
    Identificador(String),
    Acesso(Box<Expr>, String),       // ex: w.weight
    Unario(String, Box<Expr>),       // ex: -x
    Binario(Box<Expr>, String, Box<Expr>), // ex: a + b, a > b
}

