use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::types::{resolver_tipo, ErroSemantico, Tipo};

// O que o checker sabe sobre um grafo já declarado
#[derive(Debug, Clone, Default)]
pub struct InfoGrafo {
    pub nos: HashMap<String, Option<String>>,        // nó -> NodeType (None = nó sem tipo)
    pub ordem: Vec<String>,                          // nós na ordem de declaração (mensagens estáveis)
    pub tipos: HashMap<String, Vec<(String, Tipo)>>, // NodeType -> campos, na ordem declarada
    pub arestas: HashSet<(String, String)>,          // pares direcionados; `a -- b` ocupa (a, b) e (b, a)
}

impl InfoGrafo {
    fn adicionar_no(&mut self, nome: &str, tipo: Option<String>) {
        self.nos.insert(nome.to_string(), tipo);
        self.ordem.push(nome.to_string());
    }

    // Registra a aresta; devolve false se algum dos pares já existia
    fn adicionar_aresta(&mut self, origem: &str, destino: &str, tipo: &TipoAresta) -> bool {
        let mut pares = vec![(origem.to_string(), destino.to_string())];
        if *tipo == TipoAresta::NaoDirecionada && origem != destino {
            pares.push((destino.to_string(), origem.to_string()));
        }
        if pares.iter().any(|p| self.arestas.contains(p)) {
            return false;
        }
        self.arestas.extend(pares);
        true
    }
}

pub struct Checker {
    grafos: HashMap<String, InfoGrafo>,
    // Merges que falharam: ficam registrados, mas consultas sobre eles são ignoradas (evita cascata)
    incompletos: HashSet<String>,
    erros: Vec<ErroSemantico>,
}

// Ponto de entrada: percorre o programa uma vez, na ordem em que foi escrito,
// acumulando todos os erros em vez de parar no primeiro
pub fn checar(programa: &[Item]) -> Result<(), Vec<ErroSemantico>> {
    let mut checker = Checker::new();
    for item in programa {
        checker.checar_item(item);
    }
    if checker.erros.is_empty() {
        Ok(())
    } else {
        Err(checker.erros)
    }
}

impl Checker {
    fn new() -> Self {
        Checker { grafos: HashMap::new(), incompletos: HashSet::new(), erros: Vec::new() }
    }

    fn erro(&mut self, span: Span, msg: impl Into<String>) {
        self.erros.push(ErroSemantico::new(span, msg));
    }

    fn checar_item(&mut self, item: &Item) {
        match item {
            Item::Grafo(g) => self.checar_grafo(g),
            Item::GrafoMerge(m) => self.checar_merge(m),
            Item::Consulta(c) => self.checar_consulta(c),
        }
    }

    // ========== declaração de grafo

    fn checar_grafo(&mut self, grafo: &Grafo) {
        let nome = &grafo.nome.name;
        let duplicado = self.grafos.contains_key(nome);
        if duplicado {
            self.erro(grafo.nome.span, format!("grafo `{}` já declarado", nome));
        }

        // Todos os NodeType do grafo, para diferenciar "usado antes de declarar" de "não existe"
        let tipos_do_grafo: HashSet<&str> = grafo
            .membros
            .iter()
            .filter_map(|m| match m {
                Membro::Tipo(t) => Some(t.nome.name.as_str()),
                _ => None,
            })
            .collect();

        let mut info = InfoGrafo::default();
        for membro in &grafo.membros {
            match membro {
                Membro::Tipo(decl) => self.checar_tipo_decl(&mut info, decl),
                Membro::Node(id) => self.declarar_no(&mut info, nome, id, None),
                Membro::NodeTipado(no) => self.checar_no_tipado(&mut info, nome, no, &tipos_do_grafo),
                Membro::Edge(a) => self.checar_aresta(&mut info, nome, a),
            }
        }

        // Registrado mesmo com erros, para não gerar "grafo não declarado" em cascata
        if !duplicado {
            self.grafos.insert(nome.clone(), info);
        }
    }

    fn checar_tipo_decl(&mut self, info: &mut InfoGrafo, decl: &TipoDecl) {
        if info.tipos.contains_key(&decl.nome.name) {
            self.erro(decl.nome.span, format!("NodeType `{}` já declarado neste grafo", decl.nome.name));
            return;
        }
        let mut vistos = HashSet::new();
        let mut campos = Vec::new();
        for campo in &decl.campos {
            if !vistos.insert(campo.nome.name.as_str()) {
                self.erro(
                    campo.nome.span,
                    format!("campo `{}` repetido em `{}`", campo.nome.name, decl.nome.name),
                );
                continue;
            }
            match resolver_tipo(&campo.tipo) {
                Ok(t) => campos.push((campo.nome.name.clone(), t)),
                Err(e) => self.erros.push(e),
            }
        }
        info.tipos.insert(decl.nome.name.clone(), campos);
    }

    fn declarar_no(&mut self, info: &mut InfoGrafo, grafo: &str, id: &Ident, tipo: Option<String>) {
        if info.nos.contains_key(&id.name) {
            self.erro(id.span, format!("nó `{}` já declarado em `{}`", id.name, grafo));
        } else {
            info.adicionar_no(&id.name, tipo);
        }
    }

    fn checar_no_tipado(
        &mut self,
        info: &mut InfoGrafo,
        grafo: &str,
        no: &NoTipado,
        tipos_do_grafo: &HashSet<&str>,
    ) {
        let campos = match info.tipos.get(&no.tipo.name) {
            Some(c) => c.clone(),
            None => {
                let msg = if tipos_do_grafo.contains(no.tipo.name.as_str()) {
                    format!("NodeType `{}` usado antes de ser declarado", no.tipo.name)
                } else {
                    format!("NodeType `{}` não declarado em `{}`", no.tipo.name, grafo)
                };
                self.erro(no.tipo.span, msg);
                self.declarar_no(info, grafo, &no.nome, None);
                return;
            }
        };
        self.declarar_no(info, grafo, &no.nome, Some(no.tipo.name.clone()));

        let mut informados = HashSet::new();
        for atr in &no.atributos {
            if !informados.insert(atr.nome.name.as_str()) {
                self.erro(atr.nome.span, format!("atributo `{}` informado mais de uma vez", atr.nome.name));
                continue;
            }
            let Some((_, esperado)) = campos.iter().find(|(n, _)| *n == atr.nome.name) else {
                self.erro(atr.nome.span, format!("`{}` não tem o campo `{}`", no.tipo.name, atr.nome.name));
                continue;
            };
            let Some(encontrado) = tipo_literal(&atr.valor) else {
                self.erro(
                    atr.valor.span,
                    format!("valor do atributo `{}` deve ser um literal", atr.nome.name),
                );
                continue;
            };
            // Campo Option<T> aceita literal de T
            let aceito = match esperado {
                Tipo::Option(t) => **t == encontrado,
                t => *t == encontrado,
            };
            if !aceito {
                self.erro(
                    atr.valor.span,
                    format!("atributo `{}`: esperado {}, encontrado {}", atr.nome.name, esperado, encontrado),
                );
            }
        }

        // Campos que não são Option são obrigatórios
        for (campo, tipo) in &campos {
            if !matches!(tipo, Tipo::Option(_)) && !informados.contains(campo.as_str()) {
                self.erro(no.span, format!("nó `{}`: falta o campo obrigatório `{}`", no.nome.name, campo));
            }
        }
    }

    fn checar_aresta(&mut self, info: &mut InfoGrafo, grafo: &str, a: &Aresta) {
        let pontas = if a.origem.name == a.destino.name { vec![&a.origem] } else { vec![&a.origem, &a.destino] };
        let mut ok = true;
        for no in pontas {
            if !info.nos.contains_key(&no.name) {
                self.erro(no.span, format!("nó `{}` não declarado em `{}` antes desta aresta", no.name, grafo));
                ok = false;
            }
        }
        if ok && !info.adicionar_aresta(&a.origem.name, &a.destino.name, &a.tipo) {
            self.erro(
                a.span,
                format!(
                    "aresta `{} {} {}` repete uma aresta já declarada em `{}`",
                    a.origem.name,
                    seta(&a.tipo),
                    a.destino.name,
                    grafo
                ),
            );
        }
    }

    // ========== merge

    fn checar_merge(&mut self, m: &GrafoMerge) {
        let duplicado = self.grafos.contains_key(&m.nome.name);
        if duplicado {
            self.erro(m.nome.span, format!("grafo `{}` já declarado", m.nome.name));
        }

        let mut valido = true;
        for g in [&m.g1, &m.g2] {
            if !self.grafos.contains_key(&g.name) {
                self.erro(g.span, format!("grafo `{}` não declarado", g.name));
                valido = false;
            } else if self.incompletos.contains(&g.name) {
                valido = false;
            }
        }
        if valido && m.g1.name == m.g2.name {
            self.erro(m.g2.span, format!("merge de `{}` com ele mesmo", m.g1.name));
            valido = false;
        }

        let mut info = InfoGrafo::default();
        if valido {
            info = self.unir(m);
            for ponte in &m.pontes {
                self.checar_ponte(&mut info, m, ponte);
            }
        }

        if !duplicado {
            self.grafos.insert(m.nome.name.clone(), info);
            if !valido {
                self.incompletos.insert(m.nome.name.clone());
            }
        }
    }

    // União de g1 e g2: nó em comum é erro, e NodeType de mesmo nome precisa ter os mesmos campos
    fn unir(&mut self, m: &GrafoMerge) -> InfoGrafo {
        let g1 = &self.grafos[&m.g1.name];
        let g2 = &self.grafos[&m.g2.name];
        let mut info = g1.clone();
        let mut erros = Vec::new();

        for no in &g2.ordem {
            if info.nos.contains_key(no) {
                erros.push(ErroSemantico::new(
                    m.g2.span,
                    format!("nó `{}` existe em `{}` e em `{}`", no, m.g1.name, m.g2.name),
                ));
            } else {
                info.adicionar_no(no, g2.nos[no].clone());
            }
        }

        let mut nomes: Vec<&String> = g2.tipos.keys().collect();
        nomes.sort();
        for nome in nomes {
            let campos = &g2.tipos[nome];
            match info.tipos.get(nome) {
                Some(existentes) if !mesmos_campos(existentes, campos) => erros.push(ErroSemantico::new(
                    m.g2.span,
                    format!("NodeType `{}` definido de forma diferente em `{}` e em `{}`", nome, m.g1.name, m.g2.name),
                )),
                Some(_) => {}
                None => {
                    info.tipos.insert(nome.clone(), campos.clone());
                }
            }
        }

        info.arestas.extend(g2.arestas.iter().cloned());
        self.erros.extend(erros);
        info
    }

    fn checar_ponte(&mut self, info: &mut InfoGrafo, m: &GrafoMerge, p: &Ponte) {
        let mut ok = true;
        for g in [&p.g_origem, &p.g_destino] {
            if g.name != m.g1.name && g.name != m.g2.name {
                self.erro(
                    g.span,
                    format!("ponte deve ligar `{}` e `{}`, não `{}`", m.g1.name, m.g2.name, g.name),
                );
                ok = false;
            }
        }
        if !ok {
            return;
        }
        if p.g_origem.name == p.g_destino.name {
            self.erro(p.span, format!("ponte liga dois nós do mesmo grafo `{}`", p.g_origem.name));
            return;
        }
        for (g, no) in [(&p.g_origem, &p.no_origem), (&p.g_destino, &p.no_destino)] {
            let existe = self.grafos[&g.name].nos.contains_key(&no.name);
            if !existe {
                self.erro(no.span, format!("nó `{}` não existe em `{}`", no.name, g.name));
                ok = false;
            }
        }
        if ok && !info.adicionar_aresta(&p.no_origem.name, &p.no_destino.name, &p.tipo) {
            self.erro(
                p.span,
                format!(
                    "ponte `{}.{} {} {}.{}` repetida",
                    p.g_origem.name,
                    p.no_origem.name,
                    seta(&p.tipo),
                    p.g_destino.name,
                    p.no_destino.name
                ),
            );
        }
    }

    // ========== consultas

    fn checar_consulta(&mut self, c: &ExpressaoConsulta) {
        if self.incompletos.contains(&c.grafo.name) {
            return;
        }
        let Some(info) = self.grafos.get(&c.grafo.name) else {
            self.erro(c.grafo.span, format!("grafo `{}` não declarado", c.grafo.name));
            return;
        };
        let erros = &mut self.erros;

        // filter só remove arestas: toda chamada encadeada vê os mesmos nós do grafo original
        for chamada in &c.chamadas {
            match chamada {
                Chamada::ShortDist { origem, destino } => {
                    for no in [origem, destino] {
                        if !info.nos.contains_key(&no.name) {
                            erros.push(ErroSemantico::new(
                                no.span,
                                format!("nó `{}` não existe em `{}`", no.name, c.grafo.name),
                            ));
                        }
                    }
                }
                Chamada::Filter { filtro } => {
                    // `G.filter(for e in H | ...)` exige H == G
                    if filtro.fonte.name != c.grafo.name {
                        erros.push(ErroSemantico::new(
                            filtro.fonte.span,
                            format!(
                                "filter de `{}` deve percorrer o próprio grafo: `for {} in {}`",
                                c.grafo.name, filtro.variavel.name, c.grafo.name
                            ),
                        ));
                    }
                    let mut tipador = Tipador { variavel: &filtro.variavel.name, grafo: info, erros: &mut *erros };
                    if let Some(t) = tipador.tipo(&filtro.condicao) {
                        if t != Tipo::Bool {
                            erros.push(ErroSemantico::new(
                                filtro.condicao.span,
                                format!("condição do filter deve ser Bool, encontrado {}", t),
                            ));
                        }
                    }
                }
                // Resto da sintaxe antiga `G2.merge(G1)`: nenhuma regra da gramática gera esta chamada
                Chamada::Merge { grafo_alvo } => erros.push(ErroSemantico::new(
                    grafo_alvo.span,
                    "merge é uma declaração: use `Graph G3 = G1.merge(G2, ...)`",
                )),
            }
        }
    }
}

// ========== tipagem das expressões do filter

// A única variável é a aresta; os nós alcançados por origin/destination são os do grafo filtrado
struct Tipador<'a> {
    variavel: &'a str,
    grafo: &'a InfoGrafo,
    erros: &'a mut Vec<ErroSemantico>,
}

impl Tipador<'_> {
    fn erro(&mut self, span: Span, msg: impl Into<String>) {
        self.erros.push(ErroSemantico::new(span, msg));
    }

    // None = a expressão tem erro (já reportado); quem recebe None não reporta de novo
    fn tipo(&mut self, e: &Expr) -> Option<Tipo> {
        match &e.kind {
            ExprKind::Numero(_) => Some(Tipo::Number),
            ExprKind::Texto(_) => Some(Tipo::String),
            ExprKind::Bool(_) => Some(Tipo::Bool),
            ExprKind::Identificador(x) => {
                if x == self.variavel {
                    return Some(Tipo::Aresta);
                }
                if self.grafo.nos.contains_key(x) {
                    self.erro(e.span, format!("`{}` não é uma variável; para comparar com o nó use \"{}\"", x, x));
                } else {
                    self.erro(e.span, format!("variável `{}` não declarada", x));
                }
                None
            }
            ExprKind::Acesso(base, campo) => {
                let t = self.tipo(base)?;
                self.acesso(t, campo)
            }
            ExprKind::Unario(op, interno) => {
                let t = self.tipo(interno);
                let (esperado, simbolo) = match op {
                    UnOp::Not => (Tipo::Bool, "not"),
                    UnOp::Neg => (Tipo::Number, "-"),
                };
                self.exigir(interno, t, &esperado, simbolo);
                Some(esperado)
            }
            ExprKind::Binario(l, op, r) => self.binario(l, *op, r),
        }
    }

    fn acesso(&mut self, base: Tipo, campo: &Ident) -> Option<Tipo> {
        match (&base, campo.name.as_str()) {
            (Tipo::Aresta, "weight") => Some(Tipo::Number),
            (Tipo::Aresta, "origin" | "destination") => Some(Tipo::No),
            (Tipo::Aresta, "type") => Some(Tipo::String),
            (Tipo::Aresta, outro) => {
                self.erro(
                    campo.span,
                    format!("aresta não tem o campo `{}` (campos: weight, origin, destination, type)", outro),
                );
                None
            }
            (Tipo::No, _) => self.campo_de_no(campo),
            _ => {
                self.erro(campo.span, format!("{} não tem campos (acesso a `{}`)", base, campo.name));
                None
            }
        }
    }

    // `e.origin.idade` só é aceito se todo nó do grafo tem `idade`, com o mesmo tipo
    fn campo_de_no(&mut self, campo: &Ident) -> Option<Tipo> {
        let grafo = self.grafo;
        let mut encontrado: Option<&Tipo> = None;
        for no in &grafo.ordem {
            let Some(tipo_no) = &grafo.nos[no] else {
                self.erro(
                    campo.span,
                    format!("nó `{}` não tem NodeType, então não tem o campo `{}`", no, campo.name),
                );
                return None;
            };
            let campos = grafo.tipos.get(tipo_no).map(|c| c.as_slice()).unwrap_or(&[]);
            match campos.iter().find(|(n, _)| *n == campo.name) {
                None => {
                    self.erro(
                        campo.span,
                        format!("nó `{}` (`{}`) não tem o campo `{}`", no, tipo_no, campo.name),
                    );
                    return None;
                }
                Some((_, t)) => match encontrado {
                    Some(anterior) if anterior != t => {
                        self.erro(
                            campo.span,
                            format!("campo `{}` tem tipos diferentes entre os nós: {} e {}", campo.name, anterior, t),
                        );
                        return None;
                    }
                    _ => encontrado = Some(t),
                },
            }
        }
        if encontrado.is_none() {
            self.erro(campo.span, format!("grafo sem nós: não há campo `{}` para acessar", campo.name));
        }
        encontrado.cloned()
    }

    fn binario(&mut self, l: &Expr, op: BinOp, r: &Expr) -> Option<Tipo> {
        let tl = self.tipo(l);
        let tr = self.tipo(r);
        let simbolo = simbolo(op);
        match op {
            BinOp::And | BinOp::Or => {
                self.exigir(l, tl, &Tipo::Bool, simbolo);
                self.exigir(r, tr, &Tipo::Bool, simbolo);
                Some(Tipo::Bool)
            }
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                self.exigir(l, tl, &Tipo::Number, simbolo);
                self.exigir(r, tr, &Tipo::Number, simbolo);
                Some(Tipo::Number)
            }
            // Option<Number> pode ser comparado (valor ausente ⇒ falso)
            BinOp::Gt | BinOp::Gte | BinOp::Lt | BinOp::Lte => {
                self.exigir(l, tl.as_ref().map(sem_option).cloned(), &Tipo::Number, simbolo);
                self.exigir(r, tr.as_ref().map(sem_option).cloned(), &Tipo::Number, simbolo);
                Some(Tipo::Bool)
            }
            BinOp::Eq | BinOp::Neq => {
                if let (Some(a), Some(b)) = (&tl, &tr) {
                    if !comparaveis(a, b) {
                        let span = Span { start: l.span.start, end: r.span.end };
                        self.erro(span, format!("não é possível comparar {} com {} usando `{}`", a, b, simbolo));
                    }
                }
                self.checar_tipo_de_aresta(l, r);
                Some(Tipo::Bool)
            }
        }
    }

    fn exigir(&mut self, e: &Expr, t: Option<Tipo>, esperado: &Tipo, simbolo: &str) {
        if let Some(t) = t {
            if t != *esperado {
                self.erro(e.span, format!("`{}` espera {}, encontrado {}", simbolo, esperado, t));
            }
        }
    }

    // `e.type` só pode ser comparado com "->" ou "--"
    fn checar_tipo_de_aresta(&mut self, l: &Expr, r: &Expr) {
        for (a, b) in [(l, r), (r, l)] {
            if let (ExprKind::Acesso(base, campo), ExprKind::Texto(s)) = (&a.kind, &b.kind) {
                let eh_aresta = matches!(&base.kind, ExprKind::Identificador(x) if x == self.variavel);
                if eh_aresta && campo.name == "type" && s != "->" && s != "--" {
                    self.erro(b.span, format!("tipo de aresta inválido \"{}\" (esperado \"->\" ou \"--\")", s));
                }
            }
        }
    }
}

// Regras do `==`/`!=`: Option<T> compara com T, e nó compara com String pelo nome
fn comparaveis(a: &Tipo, b: &Tipo) -> bool {
    match (sem_option(a), sem_option(b)) {
        (Tipo::Aresta, _) | (_, Tipo::Aresta) => false,
        (Tipo::No, Tipo::String) | (Tipo::String, Tipo::No) => true,
        (x, y) => x == y,
    }
}

fn sem_option(t: &Tipo) -> &Tipo {
    match t {
        Tipo::Option(interno) => interno,
        t => t,
    }
}

// Tipo de um valor de atributo, se for literal; `-5` também conta
fn tipo_literal(valor: &Expr) -> Option<Tipo> {
    match &valor.kind {
        ExprKind::Numero(_) => Some(Tipo::Number),
        ExprKind::Texto(_) => Some(Tipo::String),
        ExprKind::Bool(_) => Some(Tipo::Bool),
        ExprKind::Unario(UnOp::Neg, e) if matches!(e.kind, ExprKind::Numero(_)) => Some(Tipo::Number),
        _ => None,
    }
}

// A ordem dos campos não importa: `{nome, idade}` e `{idade, nome}` são o mesmo NodeType
fn mesmos_campos(a: &[(String, Tipo)], b: &[(String, Tipo)]) -> bool {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.sort_by(|x, y| x.0.cmp(&y.0));
    b.sort_by(|x, y| x.0.cmp(&y.0));
    a == b
}

fn seta(tipo: &TipoAresta) -> &'static str {
    match tipo {
        TipoAresta::Direcionada => "->",
        TipoAresta::NaoDirecionada => "--",
    }
}

fn simbolo(op: BinOp) -> &'static str {
    match op {
        BinOp::Or => "or",
        BinOp::And => "and",
        BinOp::Eq => "==",
        BinOp::Neq => "!=",
        BinOp::Gte => ">=",
        BinOp::Lte => "<=",
        BinOp::Gt => ">",
        BinOp::Lt => "<",
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Mod => "%",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn erros(fonte: &str) -> Vec<String> {
        let programa = crate::grammar::ProgramaParser::new()
            .parse(crate::lexer::Lexer::new(fonte))
            .expect("o exemplo deveria passar no parser");
        match checar(&programa) {
            Ok(()) => vec![],
            Err(es) => es.into_iter().map(|e| e.msg).collect(),
        }
    }

    fn um_erro(fonte: &str, trecho: &str) {
        let es = erros(fonte);
        assert_eq!(es.len(), 1, "esperava 1 erro, veio {:?}", es);
        assert!(es[0].contains(trecho), "esperava `{}` em {:?}", trecho, es[0]);
    }

    const PESSOAS: &str = r#"
        Graph G {
            NodeType Pessoa { nome: String, idade: Number, ativo: Option<Bool> }
            Node = ana : Pessoa { nome: "Ana", idade: 20 }
            Node = bia : Pessoa { nome: "Bia", idade: 31, ativo: True }
            Edge = ana -> bia : 3
        }
    "#;

    fn com_filtro(cond: &str) -> String {
        format!("{}\nG.filter(for e in G | {})", PESSOAS, cond)
    }

    #[test]
    fn programa_valido_passa() {
        assert_eq!(erros(PESSOAS), Vec::<String>::new());
        assert_eq!(
            erros(&com_filtro(r#"e.weight >= 2 and not e.destination == "bia" and e.type == "->""#)),
            Vec::<String>::new()
        );
    }

    #[test]
    fn grafo_e_no_duplicados() {
        um_erro("Graph G { Node = a }\nGraph G { Node = b }", "grafo `G` já declarado");
        um_erro("Graph G { Node = {a, a} }", "nó `a` já declarado");
    }

    #[test]
    fn aresta_antes_do_no() {
        um_erro("Graph G { Node = a\n Edge = a -> b\n Node = b }", "nó `b` não declarado");
    }

    #[test]
    fn arestas_duplicadas() {
        um_erro("Graph G { Node = {a, b}\n Edge = {a -> b, a -> b : 7} }", "repete uma aresta");
        um_erro("Graph G { Node = {a, b}\n Edge = {a -- b, b -- a} }", "repete uma aresta");
        um_erro("Graph G { Node = {a, b}\n Edge = {a -- b, a -> b} }", "repete uma aresta");
        assert!(erros("Graph G { Node = {a, b}\n Edge = {a -> b, b -> a, a -> a} }").is_empty());
    }

    #[test]
    fn nodetype_e_atributos() {
        um_erro("Graph G { Node = a : P { } }", "NodeType `P` não declarado");
        um_erro("Graph G { Node = a : P { }\n NodeType P { } }", "usado antes de ser declarado");
        um_erro("Graph G { NodeType P { x: Inteiro } }", "tipo desconhecido `Inteiro`");
        um_erro("Graph G { NodeType P { x: Number }\n Node = a : P { x: 1, y: 2 } }", "não tem o campo `y`");
        um_erro("Graph G { NodeType P { x: Number }\n Node = a : P { x: 1, x: 2 } }", "mais de uma vez");
        um_erro(r#"Graph G { NodeType P { x: Number } Node = a : P { x: "vinte" } }"#, "esperado Number, encontrado String");
        um_erro("Graph G { NodeType P { x: Number }\n Node = a : P { x: 1 + 2 } }", "deve ser um literal");
        um_erro("Graph G { NodeType P { x: Number, y: Option<Bool> }\n Node = a : P { } }", "obrigatório `x`");
        assert!(erros("Graph G { NodeType P { x: Number }\n Node = a : P { x: -5 } }").is_empty());
    }

    #[test]
    fn merge() {
        let base = "Graph G1 { Node = {a, b} }\nGraph G2 { Node = {c, d} }\n";
        assert!(erros(&format!("{}Graph G3 = G1.merge(G2, G1.a -> G2.c : 10, G2.d -- G1.b)", base)).is_empty());
        um_erro(&format!("{}Graph G3 = G1.merge(G9)", base), "grafo `G9` não declarado");
        um_erro(&format!("{}Graph G3 = G1.merge(G2, G1.a -> G1.b)", base), "mesmo grafo");
        um_erro(&format!("{}Graph G3 = G1.merge(G2, G1.a -> G2.z)", base), "nó `z` não existe em `G2`");
        um_erro("Graph G1 { Node = a }\nGraph G2 { Node = a }\nGraph G3 = G1.merge(G2)", "existe em `G1` e em `G2`");
        um_erro(
            "Graph G1 { NodeType P { x: Number } }\nGraph G2 { NodeType P { x: Bool } }\nGraph G3 = G1.merge(G2)",
            "definido de forma diferente",
        );
        assert!(erros("Graph G1 { NodeType P { x: Number, y: Bool } }\nGraph G2 { NodeType P { y: Bool, x: Number } }\nGraph G3 = G1.merge(G2)").is_empty());
        // merge com erro não gera erros em cascata nas consultas
        um_erro(&format!("{}Graph G3 = G1.merge(G9)\nG3.short_dist(a, z)", base), "G9");
    }

    #[test]
    fn consultas() {
        um_erro(&format!("{}\nG.short_dist(ana, zeca)", PESSOAS), "nó `zeca` não existe");
        um_erro(&format!("{}\nH.short_dist(ana, bia)", PESSOAS), "grafo `H` não declarado");
        um_erro(&format!("{}\nG.filter(for e in H | e.weight > 1)", PESSOAS), "próprio grafo");
    }

    #[test]
    fn tipagem_do_filtro() {
        um_erro(&com_filtro("e.weight + 1"), "deve ser Bool");
        um_erro(&com_filtro("e.peso > 1"), "não tem o campo `peso`");
        um_erro(&com_filtro("x.weight >= 1"), "variável `x` não declarada");
        um_erro(&com_filtro(r#"e.weight == "a""#), "não é possível comparar Number com String");
        um_erro(&com_filtro(r#"e.type == "=>""#), "tipo de aresta inválido");
        um_erro(&com_filtro("e.origin == ana"), "use \"ana\"");
    }

    #[test]
    fn atributos_de_no_no_filtro() {
        assert!(erros(&com_filtro("e.origin.idade > 18 and e.destination.ativo == True")).is_empty());
        assert!(erros(&com_filtro("e.origin == e.destination")).is_empty());
        um_erro(&com_filtro("e.origin.email == \"x\""), "não tem o campo `email`");
        um_erro(&com_filtro("e.origin.ativo + 1 > 2"), "encontrado Option<Bool>");
        um_erro(
            "Graph G { NodeType P { x: Number }\n Node = a : P { x: 1 }\n Node = b }\nG.filter(for e in G | e.origin.x > 1)",
            "nó `b` não tem NodeType",
        );
    }
}
