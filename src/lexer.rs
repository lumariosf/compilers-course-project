use logos::Logos; 

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]    // para ignorar espaços em branco
#[logos(skip r"//[^n]*")]       // para ignorar comentarios de linha

pum enum Token {
    // keywords selecionadas
    #[token("Graph")]
    Graph, 
    #[token("Node")]
    Node, 
    #[Token("Edge")]
    Edge, 
    #[token("for")]
    For, 
    #[token("in")]
    In, 
    #[token("or")]
    Or, 
    #[token("not")]
    Not, 

    // consultas
    #[token("short_dist")]
    ShortDist, 
    #[token("filter")]
    Filter, 
    #[token("merge")]
    Merge, 

    // literais e identificadores
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    Str(String), 

    // simbolos das arestas
    #[token("->")]
    ArrowDir, 
    #[token("--")]
    ArrowUndir, 

    // delimitadores
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token(";")]
    Semicolon,
    #[token("|")]
    Pipe,
    #[token("=")]
    Equals,

    // operadores relacionais
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token(">=")]
    Gte,
    #[token("<=")]
    Lte,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,

    // operadores aritméticos
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
}