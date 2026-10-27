use std::fmt;

use crate::ast::{Span, TipoAnotado};

// Tipos semânticos usados pelo checker.
// Diferente de TipoAnotado (como o usuário ESCREVEU o tipo), aqui o tipo já foi validado.
#[derive(Debug, Clone, PartialEq)]
pub enum Tipo {
    Number,
    String,
    Bool,
    Option(Box<Tipo>),
    Aresta, // tipo da variável do filter: for e in G | ...
    No,     // e.origin / e.destination: compara com String pelo nome e dá acesso aos atributos
}

impl fmt::Display for Tipo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tipo::Number => write!(f, "Number"),
            Tipo::String => write!(f, "String"),
            Tipo::Bool => write!(f, "Bool"),
            Tipo::Option(t) => write!(f, "Option<{}>", t),
            Tipo::Aresta => write!(f, "Aresta"),
            Tipo::No => write!(f, "Nó"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErroSemantico {
    pub msg: String,
    pub span: Span,
}

impl ErroSemantico {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        ErroSemantico { msg: msg.into(), span }
    }
}

// Converte o offset em bytes de um Span em (linha, coluna), ambos começando em 1
pub fn linha_coluna(fonte: &str, offset: usize) -> (usize, usize) {
    let antes = fonte.get(..offset).unwrap_or(fonte);
    let linha = antes.matches('\n').count() + 1;
    let coluna = antes.chars().rev().take_while(|&c| c != '\n').count() + 1;
    (linha, coluna)
}

// Valida o tipo de um campo de NodeType: Number, String, Bool ou Option<T> (sem Option aninhado)
pub fn resolver_tipo(t: &TipoAnotado) -> Result<Tipo, ErroSemantico> {
    match t {
        TipoAnotado::Nome(id) => match id.name.as_str() {
            "Number" => Ok(Tipo::Number),
            "String" => Ok(Tipo::String),
            "Bool" => Ok(Tipo::Bool),
            "Option" => Err(ErroSemantico::new(
                id.span,
                "Option precisa de um argumento: Option<T>",
            )),
            outro => Err(ErroSemantico::new(
                id.span,
                format!("tipo desconhecido `{}` (esperado Number, String, Bool ou Option<T>)", outro),
            )),
        },
        TipoAnotado::Generico { nome, arg } => {
            if nome.name != "Option" {
                return Err(ErroSemantico::new(
                    nome.span,
                    format!("tipo genérico desconhecido `{}` (o único genérico é Option<T>)", nome.name),
                ));
            }
            let interno = resolver_tipo(arg)?;
            if let Tipo::Option(_) = interno {
                return Err(ErroSemantico::new(nome.span, "Option<Option<T>> não é permitido"));
            }
            Ok(Tipo::Option(Box::new(interno)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ident;

    fn nome(s: &str) -> TipoAnotado {
        TipoAnotado::Nome(Ident { name: s.to_string(), span: Span { start: 0, end: 0 } })
    }

    fn generico(s: &str, arg: TipoAnotado) -> TipoAnotado {
        TipoAnotado::Generico {
            nome: Ident { name: s.to_string(), span: Span { start: 0, end: 0 } },
            arg: Box::new(arg),
        }
    }

    #[test]
    fn linha_coluna_conta_a_partir_de_1() {
        let fonte = "Graph G {\n    Node = a\n}";
        assert_eq!(linha_coluna(fonte, 0), (1, 1));
        assert_eq!(linha_coluna(fonte, 6), (1, 7));
        assert_eq!(linha_coluna(fonte, 14), (2, 5));
    }

    #[test]
    fn resolve_tipos_primitivos_e_option() {
        assert_eq!(resolver_tipo(&nome("Number")), Ok(Tipo::Number));
        assert_eq!(resolver_tipo(&nome("String")), Ok(Tipo::String));
        assert_eq!(resolver_tipo(&nome("Bool")), Ok(Tipo::Bool));
        assert_eq!(
            resolver_tipo(&generico("Option", nome("Bool"))),
            Ok(Tipo::Option(Box::new(Tipo::Bool)))
        );
    }

    #[test]
    fn rejeita_tipos_invalidos() {
        assert!(resolver_tipo(&nome("Inteiro")).is_err());
        assert!(resolver_tipo(&nome("Option")).is_err());
        assert!(resolver_tipo(&generico("Lista", nome("Number"))).is_err());
        assert!(resolver_tipo(&generico("Option", generico("Option", nome("Bool")))).is_err());
    }
}
