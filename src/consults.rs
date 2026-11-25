// Funções nativas de GRAPH: consultas que rodam um algoritmo sobre o grafo.
//
//   G.short_dist(a, b)   (keyword, em interpreter.rs)  Dijkstra: distância mínima
//   G.short_path(a, b)   Dijkstra: lista de arestas do menor caminho
//   G.connected(a, b)    BFS: existe caminho de a até b? (Bool, nunca dá erro)
//
// Para criar uma função nova: (1) acrescente a assinatura em `assinatura` e o nome em `NOMES`,
// (2) acrescente um braço em `executar_nativa`. O parser e o checker não precisam mudar.

use std::collections::{HashMap, VecDeque};

use crate::ast::*;
use crate::interpreter::{calcular_caminho_minimo, ErroExecucao, GrafoRuntime, ResultadoConsulta};
use crate::types::Tipo;

pub const NOMES: &[&str] = &["short_path", "connected"];

// (quantidade de nós esperados como argumento, tipo do resultado). None = a função não existe.
// Para `short_path` o resultado é a lista de arestas, que o checker chama de `Grafo`.
pub fn assinatura(nome: &str) -> Option<(usize, Tipo)> {
    match nome {
        "short_path" => Some((2, Tipo::Grafo)),
        "connected" => Some((2, Tipo::Bool)),
        _ => None,
    }
}

pub fn executar_nativa(
    grafo: &GrafoRuntime,
    nome_grafo: &str,
    nome: &Ident,
    args: &[Ident],
    span: Span,
) -> Result<ResultadoConsulta, ErroExecucao> {
    let Some((aridade, _)) = assinatura(&nome.name) else {
        return Err(ErroExecucao::new(nome.span, format!("função nativa `{}` desconhecida", nome.name)));
    };
    if args.len() != aridade {
        return Err(ErroExecucao::new(
            nome.span,
            format!("`{}` espera {} argumento(s), recebeu {}", nome.name, aridade, args.len()),
        ));
    }
    for no in args {
        if !grafo.nos.contains_key(&no.name) {
            return Err(ErroExecucao::new(
                span,
                format!("nó `{}` não existe no grafo `{}`", no.name, grafo.nome),
            ));
        }
    }

    match nome.name.as_str() {
        "short_path" => {
            let (_, arestas) = calcular_caminho_minimo(grafo, &args[0].name, &args[1].name, span)?;
            Ok(ResultadoConsulta::Caminho { grafo: nome_grafo.to_string(), arestas })
        }
        "connected" => Ok(ResultadoConsulta::Booleano(existe_caminho(grafo, &args[0].name, &args[1].name))),
        _ => unreachable!("assinatura() já recusou nomes desconhecidos"),
    }
}

// ========== BFS: existe caminho de `origem` até `destino`? (respeita a direção das arestas `->`)

fn existe_caminho(grafo: &GrafoRuntime, origem: &str, destino: &str) -> bool {
    if origem == destino {
        return true;
    }
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for a in &grafo.arestas {
        adj.entry(a.origem.as_str()).or_default().push(a.destino.as_str());
        if a.tipo == TipoAresta::NaoDirecionada {
            adj.entry(a.destino.as_str()).or_default().push(a.origem.as_str());
        }
    }

    let mut visitados: HashMap<&str, ()> = HashMap::new();
    let mut fila: VecDeque<&str> = VecDeque::new();
    visitados.insert(origem, ());
    fila.push_back(origem);

    while let Some(no) = fila.pop_front() {
        if let Some(vizinhos) = adj.get(no) {
            for &v in vizinhos {
                if v == destino {
                    return true;
                }
                if visitados.insert(v, ()).is_none() {
                    fila.push_back(v);
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::interpreter::{Interpretador, ResultadoConsulta};

    fn rodar(fonte: &str) -> Result<Vec<ResultadoConsulta>, String> {
        let programa = crate::grammar::ProgramaParser::new()
            .parse(crate::lexer::Lexer::new(fonte))
            .map_err(|e| format!("Erro de parsing: {:?}", e))?;
        crate::checker::checar(&programa).map_err(|es| {
            format!("Erros semânticos: {:?}", es.iter().map(|e| &e.msg).collect::<Vec<_>>())
        })?;
        Interpretador::new().executar(&programa).map_err(|e| e.msg)
    }

    const G: &str = r#"
        Graph G {
            Node = {a, b, c, d, e}
            Edge = { a -> b : 2, b -> c : 3, a -> c : 10, c -- d : 1 }
        }
    "#;

    fn com(consulta: &str) -> String {
        format!("{}\n{}", G, consulta)
    }

    #[test]
    fn short_path_lista_as_arestas_do_menor_caminho() {
        let res = rodar(&com("G.short_path(a, d)")).unwrap();
        match &res[0] {
            ResultadoConsulta::Caminho { arestas, .. } => {
                let texto: Vec<String> = arestas.iter().map(|a| a.to_string()).collect();
                assert_eq!(texto, vec!["a -> b : 2", "b -> c : 3", "c -- d : 1"]);
            }
            outro => panic!("esperava Caminho, veio {:?}", outro),
        }
    }

    #[test]
    fn short_path_orienta_aresta_nao_direcionada_no_sentido_percorrido() {
        let res = rodar(&com("G.short_path(d, c)")).unwrap();
        match &res[0] {
            ResultadoConsulta::Caminho { arestas, .. } => assert_eq!(arestas[0].to_string(), "d -- c : 1"),
            outro => panic!("esperava Caminho, veio {:?}", outro),
        }
    }

    #[test]
    fn short_path_do_no_para_ele_mesmo_e_vazio() {
        let res = rodar(&com("G.short_path(a, a)")).unwrap();
        match &res[0] {
            ResultadoConsulta::Caminho { arestas, .. } => assert!(arestas.is_empty()),
            outro => panic!("esperava Caminho, veio {:?}", outro),
        }
    }

    #[test]
    fn short_path_desconectado_e_erro() {
        let erro = rodar(&com("G.short_path(a, e)")).unwrap_err();
        assert!(erro.contains("não estão conectados"));
    }

    #[test]
    fn connected_nunca_da_erro() {
        assert_eq!(rodar(&com("G.connected(a, d)")).unwrap(), vec![ResultadoConsulta::Booleano(true)]);
        assert_eq!(rodar(&com("G.connected(d, a)")).unwrap(), vec![ResultadoConsulta::Booleano(false)]);
        assert_eq!(rodar(&com("G.connected(a, e)")).unwrap(), vec![ResultadoConsulta::Booleano(false)]);
        assert_eq!(rodar(&com("G.connected(e, e)")).unwrap(), vec![ResultadoConsulta::Booleano(true)]);
    }

    #[test]
    fn checker_recusa_nativa_desconhecida_aridade_e_no_inexistente() {
        assert!(rodar(&com("G.voar(a)")).unwrap_err().contains("desconhecida"));
        assert!(rodar(&com("G.connected(a)")).unwrap_err().contains("espera 2 argumento"));
        assert!(rodar(&com("G.connected(a, zzz)")).unwrap_err().contains("não existe"));
    }
}
