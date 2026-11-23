#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Grafo(Grafo),
    GrafoMerge(GrafoMerge),
    Consulta(ExpressaoConsulta),
    Let(LetDecl),
    Print(PrintStmt),
}

// let nome = expr;   (imutável: o nome vale sempre o mesmo valor)
#[derive(Debug, Clone, PartialEq)]
pub struct LetDecl {
    pub nome: Ident,
    pub valor: Expr,
    pub span: Span,
}

// print(e1, e2, ...);
#[derive(Debug, Clone, PartialEq)]
pub struct PrintStmt {
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span { pub start: usize, pub end: usize }

// Um nome com a posição onde apareceu
#[derive(Debug, Clone, PartialEq)]
pub struct Ident { pub name: String, pub span: Span }

#[derive(Debug, Clone, PartialEq)]
pub struct Grafo {
    pub nome: Ident,
    pub membros: Vec<Membro>,   // na ORDEM em que foram escritos
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrafoMerge {
    pub nome: Ident,
    pub g1: Ident,
    pub g2: Ident,
    pub pontes: Vec<Ponte>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ponte {
    pub g_origem: Ident,
    pub no_origem: Ident,
    pub tipo: TipoAresta,
    pub g_destino: Ident,
    pub no_destino: Ident,
    pub peso: Option<f64>,
    pub span: Span,
}

// Substitui ItemGrafoParcial: agora é parte real do AST
#[derive(Debug, Clone, PartialEq)]
pub enum Membro {
     Node(Ident),             // nó sem tipo: Node = a
    NodeTipado(NoTipado),    // Node = ana : Pessoa { ... }
    Edge(Aresta),
    Tipo(TipoDecl),          // NodeType Pessoa { ... }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Aresta {
    pub origem: Ident,
    pub destino: Ident,
    pub tipo: TipoAresta,
    pub peso: Option<f64>,      // None = o usuário não escreveu
    pub span: Span,
}


#[derive(Debug, Clone, PartialEq)]
pub enum TipoAresta {
    Direcionada,   // ->
    NaoDirecionada, // --
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressaoConsulta {
    pub grafo: Ident,
    pub chamadas: Vec<Chamada>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Chamada {
    ShortDist { origem: Ident, destino: Ident },
    Filter { filtro: Filtro },
    Merge { grafo_alvo: Ident },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filtro {
    pub variavel: Ident,
    pub fonte: Ident,
    pub condicao: Expr,
}


//A ideia é que Expr sempre carrega o span (e.span), e ExprKind diz que forma a expressão tem.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp { Or, And, Eq, Neq, Gte, Lte, Gt, Lt, Add, Sub, Mul, Div, Mod }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp { Not, Neg }

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Numero(f64),
    Texto(String),
    Bool(bool),
    Identificador(String),
    Acesso(Box<Expr>, Ident),
    Unario(UnOp, Box<Expr>),
    Binario(Box<Expr>, BinOp, Box<Expr>),
    // G.short_dist(a, b), G.filter(...): a chamada vem depois do que está à esquerda (encadeável).
    // A Chamada fica em Box porque o Filtro contém uma Expr (sem Box o tipo teria tamanho infinito).
    Consulta(Box<Expr>, Box<Chamada>),
}

impl Expr {
    // O span de "a + b" vai do início de a até o fim de b
    pub fn binario(l: Expr, op: BinOp, r: Expr) -> Expr {
        let span = Span { start: l.span.start, end: r.span.end };
        Expr { kind: ExprKind::Binario(Box::new(l), op, Box::new(r)), span }
    }

    // `G.filter(...).short_dist(a, b)` vira ExpressaoConsulta { grafo: G, chamadas: [filter, short_dist] },
    // o formato que o checker e o interpretador já sabem tratar. None se a base não for o nome de um grafo.
    pub fn como_consulta(&self) -> Option<ExpressaoConsulta> {
        let mut chamadas: Vec<Chamada> = Vec::new();
        let mut atual: &Expr = self;
        loop {
            match &atual.kind {
                ExprKind::Consulta(base, chamada) => {
                    chamadas.push((**chamada).clone());
                    atual = &**base;
                }
                ExprKind::Identificador(nome) => {
                    chamadas.reverse();
                    return Some(ExpressaoConsulta {
                        grafo: Ident { name: nome.clone(), span: atual.span },
                        chamadas,
                    });
                }
                _ => return None,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TipoDecl {
    pub nome: Ident,
    pub campos: Vec<CampoDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CampoDecl {
    pub nome: Ident,
    pub tipo: TipoAnotado,
    pub span: Span,
}

// O tipo como o usuário escreveu: Number, ou Option<Bool>
#[derive(Debug, Clone, PartialEq)]
pub enum TipoAnotado {
    Nome(Ident),
    Generico { nome: Ident, arg: Box<TipoAnotado> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoTipado {
    pub nome: Ident,
    pub tipo: Ident,
    pub atributos: Vec<Atributo>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atributo {
    pub nome: Ident,
    pub valor: Expr,
    pub span: Span,
}
