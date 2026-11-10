use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ErroExecucao {
    pub msg: String,
    pub span: Span,
}

impl ErroExecucao {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        ErroExecucao {
            msg: msg.into(),
            span,
        }
    }
}

impl fmt::Display for ErroExecucao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ErroExecucao {}

#[derive(Debug, Clone, PartialEq)]
pub struct Aviso {
    pub msg: String,
    pub span: Span,
}

impl Aviso {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        Aviso {
            msg: msg.into(),
            span,
        }
    }
}

impl fmt::Display for Aviso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Valor {
    Numero(f64),
    Texto(String),
    Bool(bool),
    No(String),
    Ausente,
}

impl fmt::Display for Valor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Valor::Numero(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Valor::Texto(s) => write!(f, "\"{}\"", s),
            Valor::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Valor::No(n) => write!(f, "{}", n),
            Valor::Ausente => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoRuntime {
    pub nome: String,
    pub tipo: Option<String>,
    pub atributos: HashMap<String, Valor>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArestaRuntime {
    pub origem: String,
    pub destino: String,
    pub tipo: TipoAresta,
    pub peso: f64,
}

impl fmt::Display for ArestaRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seta = match self.tipo {
            TipoAresta::Direcionada => "->",
            TipoAresta::NaoDirecionada => "--",
        };
        if self.peso.fract() == 0.0 {
            write!(f, "{} {} {} : {}", self.origem, seta, self.destino, self.peso as i64)
        } else {
            write!(f, "{} {} {} : {}", self.origem, seta, self.destino, self.peso)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrafoRuntime {
    pub nome: String,
    pub nos: HashMap<String, NoRuntime>,
    pub arestas: Vec<ArestaRuntime>,
}

impl GrafoRuntime {
    pub fn new(nome: String) -> Self {
        GrafoRuntime {
            nome,
            nos: HashMap::new(),
            arestas: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResultadoConsulta {
    Distancia {
        grafo: String,
        origem: String,
        destino: String,
        distancia: f64,
    },
    Filtro {
        grafo: String,
        arestas: Vec<ArestaRuntime>,
    },
}

impl fmt::Display for ResultadoConsulta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultadoConsulta::Distancia { distancia, .. } => {
                if distancia.fract() == 0.0 {
                    write!(f, "{}", *distancia as i64)
                } else {
                    write!(f, "{}", distancia)
                }
            }
            ResultadoConsulta::Filtro { arestas, .. } => {
                if arestas.is_empty() {
                    write!(f, "[]")
                } else {
                    for (i, a) in arestas.iter().enumerate() {
                        if i + 1 == arestas.len() {
                            write!(f, "{}", a)?;
                        } else {
                            writeln!(f, "{}", a)?;
                        }
                    }
                    Ok(())
                }
            }
        }
    }
}

pub fn avisos(programa: &[Item]) -> Vec<Aviso> {
    let mut lista = Vec::new();

    // 1. Grafos declarados mas nunca utilizados em consultas ou merges
    let mut grafos_declarados: Vec<&Ident> = Vec::new();
    let mut grafos_usados: HashSet<String> = HashSet::new();

    for item in programa {
        match item {
            Item::Grafo(g) => {
                grafos_declarados.push(&g.nome);
            }
            Item::GrafoMerge(m) => {
                grafos_declarados.push(&m.nome);
                grafos_usados.insert(m.g1.name.clone());
                grafos_usados.insert(m.g2.name.clone());
            }
            Item::Consulta(c) => {
                grafos_usados.insert(c.grafo.name.clone());
                for chamada in &c.chamadas {
                    if let Chamada::Filter { filtro } = chamada {
                        grafos_usados.insert(filtro.fonte.name.clone());
                    }
                }
            }
        }
    }

    for g_id in grafos_declarados {
        if !grafos_usados.contains(&g_id.name) {
            lista.push(Aviso::new(
                g_id.span,
                format!("grafo `{}` declarado mas não utilizado", g_id.name),
            ));
        }
    }

    // 2. Nós declarados em grafos mas nunca utilizados (sem arestas, pontes ou consultas)
    for item in programa {
        if let Item::Grafo(g) = item {
            let mut nos_declarados: Vec<&Ident> = Vec::new();
            let mut nos_usados: HashSet<String> = HashSet::new();

            for membro in &g.membros {
                match membro {
                    Membro::Node(id) => nos_declarados.push(id),
                    Membro::NodeTipado(no) => nos_declarados.push(&no.nome),
                    Membro::Edge(a) => {
                        nos_usados.insert(a.origem.name.clone());
                        nos_usados.insert(a.destino.name.clone());
                    }
                    Membro::Tipo(_) => {}
                }
            }

            // Uso em pontes de merge
            for outro in programa {
                if let Item::GrafoMerge(m) = outro {
                    for p in &m.pontes {
                        if p.g_origem.name == g.nome.name {
                            nos_usados.insert(p.no_origem.name.clone());
                        }
                        if p.g_destino.name == g.nome.name {
                            nos_usados.insert(p.no_destino.name.clone());
                        }
                    }
                }
            }

            // Uso em consultas short_dist
            for outro in programa {
                if let Item::Consulta(c) = outro {
                    for chamada in &c.chamadas {
                        if let Chamada::ShortDist { origem, destino } = chamada {
                            nos_usados.insert(origem.name.clone());
                            nos_usados.insert(destino.name.clone());
                        }
                    }
                }
            }

            for no_id in nos_declarados {
                if !nos_usados.contains(&no_id.name) {
                    lista.push(Aviso::new(
                        no_id.span,
                        format!(
                            "nó `{}` no grafo `{}` declarado mas não utilizado",
                            no_id.name, g.nome.name
                        ),
                    ));
                }
            }
        }
    }

    // 3. Variáveis de filtro declaradas mas não utilizadas na condição do filter
    for item in programa {
        if let Item::Consulta(c) = item {
            for chamada in &c.chamadas {
                if let Chamada::Filter { filtro } = chamada {
                    if !expr_utiliza_var(&filtro.condicao, &filtro.variavel.name) {
                        lista.push(Aviso::new(
                            filtro.variavel.span,
                            format!(
                                "variável `{}` no filtro de `{}` declarada mas não utilizada",
                                filtro.variavel.name, c.grafo.name
                            ),
                        ));
                    }
                }
            }
        }
    }

    lista
}

fn expr_utiliza_var(expr: &Expr, var: &str) -> bool {
    match &expr.kind {
        ExprKind::Identificador(id) => id == var,
        ExprKind::Acesso(base, _) => expr_utiliza_var(base, var),
        ExprKind::Unario(_, inner) => expr_utiliza_var(inner, var),
        ExprKind::Binario(l, _, r) => expr_utiliza_var(l, var) || expr_utiliza_var(r, var),
        ExprKind::Numero(_) | ExprKind::Texto(_) | ExprKind::Bool(_) => false,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interpretador {
    grafos: HashMap<String, GrafoRuntime>,
}

impl Default for Interpretador {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpretador {
    pub fn new() -> Self {
        Interpretador {
            grafos: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn grafos(&self) -> &HashMap<String, GrafoRuntime> {
        &self.grafos
    }

    #[allow(dead_code)]
    pub fn executar(&mut self, programa: &[Item]) -> Result<Vec<ResultadoConsulta>, ErroExecucao> {
        let mut resultados = Vec::new();
        for item in programa {
            if let Some(res) = self.executar_item(item)? {
                resultados.push(res);
            }
        }
        Ok(resultados)
    }

    pub fn executar_item(&mut self, item: &Item) -> Result<Option<ResultadoConsulta>, ErroExecucao> {
        match item {
            Item::Grafo(g) => {
                self.executar_grafo(g)?;
                Ok(None)
            }
            Item::GrafoMerge(m) => {
                self.executar_merge(m)?;
                Ok(None)
            }
            Item::Consulta(c) => {
                let res = self.executar_consulta(c)?;
                Ok(Some(res))
            }
        }
    }

    fn executar_grafo(&mut self, g: &Grafo) -> Result<(), ErroExecucao> {
        let mut runtime = GrafoRuntime::new(g.nome.name.clone());

        for membro in &g.membros {
            match membro {
                Membro::Node(id) => {
                    runtime.nos.insert(
                        id.name.clone(),
                        NoRuntime {
                            nome: id.name.clone(),
                            tipo: None,
                            atributos: HashMap::new(),
                        },
                    );
                }
                Membro::NodeTipado(no) => {
                    let mut atributos = HashMap::new();
                    for atr in &no.atributos {
                        let valor = avaliar_literal(&atr.valor)?;
                        atributos.insert(atr.nome.name.clone(), valor);
                    }
                    runtime.nos.insert(
                        no.nome.name.clone(),
                        NoRuntime {
                            nome: no.nome.name.clone(),
                            tipo: Some(no.tipo.name.clone()),
                            atributos,
                        },
                    );
                }
                Membro::Edge(a) => {
                    runtime.arestas.push(ArestaRuntime {
                        origem: a.origem.name.clone(),
                        destino: a.destino.name.clone(),
                        tipo: a.tipo.clone(),
                        peso: a.peso.unwrap_or(1.0),
                    });
                }
                Membro::Tipo(_) => {}
            }
        }

        self.grafos.insert(g.nome.name.clone(), runtime);
        Ok(())
    }

    fn executar_merge(&mut self, m: &GrafoMerge) -> Result<(), ErroExecucao> {
        let g1 = self
            .grafos
            .get(&m.g1.name)
            .ok_or_else(|| ErroExecucao::new(m.g1.span, format!("grafo `{}` não encontrado", m.g1.name)))?;
        let g2 = self
            .grafos
            .get(&m.g2.name)
            .ok_or_else(|| ErroExecucao::new(m.g2.span, format!("grafo `{}` não encontrado", m.g2.name)))?;

        let mut runtime = GrafoRuntime::new(m.nome.name.clone());

        // Clona nós de G1 e G2
        for (nome, no) in &g1.nos {
            runtime.nos.insert(nome.clone(), no.clone());
        }
        for (nome, no) in &g2.nos {
            runtime.nos.insert(nome.clone(), no.clone());
        }

        // Clona arestas de G1 e G2
        runtime.arestas.extend(g1.arestas.iter().cloned());
        runtime.arestas.extend(g2.arestas.iter().cloned());

        // Adiciona as pontes
        for p in &m.pontes {
            runtime.arestas.push(ArestaRuntime {
                origem: p.no_origem.name.clone(),
                destino: p.no_destino.name.clone(),
                tipo: p.tipo.clone(),
                peso: p.peso.unwrap_or(1.0),
            });
        }

        self.grafos.insert(m.nome.name.clone(), runtime);
        Ok(())
    }

    fn executar_consulta(&self, c: &ExpressaoConsulta) -> Result<ResultadoConsulta, ErroExecucao> {
        let grafo_inicial = self.grafos.get(&c.grafo.name).ok_or_else(|| {
            ErroExecucao::new(c.grafo.span, format!("grafo `{}` não encontrado", c.grafo.name))
        })?;

        let mut grafo_atual = grafo_inicial.clone();
        let mut ultimo_resultado = None;

        for chamada in &c.chamadas {
            match chamada {
                Chamada::Filter { filtro } => {
                    let mut arestas_filtradas = Vec::new();
                    for aresta in &grafo_atual.arestas {
                        let contexto = ContextoAresta {
                            variavel: &filtro.variavel.name,
                            aresta,
                            grafo: &grafo_atual,
                        };
                        let cond = avaliar_expr(&filtro.condicao, Some(&contexto))?;
                        if let Valor::Bool(true) = cond {
                            arestas_filtradas.push(aresta.clone());
                        }
                    }
                    grafo_atual.arestas = arestas_filtradas.clone();
                    ultimo_resultado = Some(ResultadoConsulta::Filtro {
                        grafo: c.grafo.name.clone(),
                        arestas: arestas_filtradas,
                    });
                }
                Chamada::ShortDist { origem, destino } => {
                    let d = calcular_short_dist(
                        &grafo_atual,
                        &origem.name,
                        &destino.name,
                        c.grafo.span,
                    )?;
                    ultimo_resultado = Some(ResultadoConsulta::Distancia {
                        grafo: c.grafo.name.clone(),
                        origem: origem.name.clone(),
                        destino: destino.name.clone(),
                        distancia: d,
                    });
                }
                Chamada::Merge { grafo_alvo } => {
                    return Err(ErroExecucao::new(
                        grafo_alvo.span,
                        "merge é uma declaração: use `Graph G3 = G1.merge(G2, ...)`",
                    ));
                }
            }
        }

        ultimo_resultado.ok_or_else(|| {
            ErroExecucao::new(c.grafo.span, "consulta sem chamadas")
        })
    }
}

// ========== Contexto para avaliação do filter

struct ContextoAresta<'a> {
    variavel: &'a str,
    aresta: &'a ArestaRuntime,
    grafo: &'a GrafoRuntime,
}

// ========== Avaliação de Expressões

fn avaliar_literal(expr: &Expr) -> Result<Valor, ErroExecucao> {
    match &expr.kind {
        ExprKind::Numero(n) => Ok(Valor::Numero(*n)),
        ExprKind::Texto(s) => Ok(Valor::Texto(s.clone())),
        ExprKind::Bool(b) => Ok(Valor::Bool(*b)),
        ExprKind::Unario(UnOp::Neg, e) => {
            if let ExprKind::Numero(n) = &e.kind {
                Ok(Valor::Numero(-n))
            } else {
                Err(ErroExecucao::new(expr.span, "valor de atributo deve ser um literal"))
            }
        }
        _ => Err(ErroExecucao::new(expr.span, "valor de atributo deve ser um literal")),
    }
}

fn avaliar_expr(expr: &Expr, ctx: Option<&ContextoAresta>) -> Result<Valor, ErroExecucao> {
    match &expr.kind {
        ExprKind::Numero(n) => Ok(Valor::Numero(*n)),
        ExprKind::Texto(s) => Ok(Valor::Texto(s.clone())),
        ExprKind::Bool(b) => Ok(Valor::Bool(*b)),
        ExprKind::Identificador(id) => {
            if let Some(c) = ctx {
                if id == c.variavel {
                    return Ok(Valor::Texto(id.clone()));
                }
            }
            Err(ErroExecucao::new(
                expr.span,
                format!("identificador `{}` não resolvido no contexto", id),
            ))
        }
        ExprKind::Acesso(base, campo) => {
            if let ExprKind::Identificador(ref base_id) = base.kind {
                if let Some(c) = ctx {
                    if base_id == c.variavel {
                        return match campo.name.as_str() {
                            "weight" => Ok(Valor::Numero(c.aresta.peso)),
                            "origin" => Ok(Valor::No(c.aresta.origem.clone())),
                            "destination" => Ok(Valor::No(c.aresta.destino.clone())),
                            "type" => {
                                let seta = match c.aresta.tipo {
                                    TipoAresta::Direcionada => "->",
                                    TipoAresta::NaoDirecionada => "--",
                                };
                                Ok(Valor::Texto(seta.to_string()))
                            }
                            outro => Err(ErroExecucao::new(
                                campo.span,
                                format!("aresta não possui o campo `{}`", outro),
                            )),
                        };
                    }
                }
            }

            let base_val = avaliar_expr(base, ctx)?;
            match base_val {
                Valor::No(nome_no) => {
                    let c = ctx.ok_or_else(|| {
                        ErroExecucao::new(campo.span, "acesso a nó fora do contexto de filtro")
                    })?;
                    let no = c.grafo.nos.get(&nome_no).ok_or_else(|| {
                        ErroExecucao::new(campo.span, format!("nó `{}` não encontrado no grafo", nome_no))
                    })?;
                    match no.atributos.get(&campo.name) {
                        Some(val) => Ok(val.clone()),
                        None => Ok(Valor::Ausente),
                    }
                }
                _ => Err(ErroExecucao::new(
                    campo.span,
                    format!("não é possível acessar campo `{}` neste valor", campo.name),
                )),
            }
        }
        ExprKind::Unario(op, inner) => {
            let val = avaliar_expr(inner, ctx)?;
            match (op, val) {
                (UnOp::Not, Valor::Bool(b)) => Ok(Valor::Bool(!b)),
                (UnOp::Neg, Valor::Numero(n)) => Ok(Valor::Numero(-n)),
                (UnOp::Not, outro) => Err(ErroExecucao::new(
                    inner.span,
                    format!("operador `not` espera Bool, encontrado {}", outro),
                )),
                (UnOp::Neg, outro) => Err(ErroExecucao::new(
                    inner.span,
                    format!("operador `-` espera Number, encontrado {}", outro),
                )),
            }
        }
        ExprKind::Binario(l, op, r) => {
            // Operadores com curto-circuito
            if *op == BinOp::And {
                let val_l = avaliar_expr(l, ctx)?;
                match val_l {
                    Valor::Bool(false) => return Ok(Valor::Bool(false)),
                    Valor::Bool(true) => {
                        let val_r = avaliar_expr(r, ctx)?;
                        return match val_r {
                            Valor::Bool(b) => Ok(Valor::Bool(b)),
                            outro => Err(ErroExecucao::new(
                                r.span,
                                format!("operador `and` espera Bool, encontrado {}", outro),
                            )),
                        };
                    }
                    outro => {
                        return Err(ErroExecucao::new(
                            l.span,
                            format!("operador `and` espera Bool, encontrado {}", outro),
                        ));
                    }
                }
            }

            if *op == BinOp::Or {
                let val_l = avaliar_expr(l, ctx)?;
                match val_l {
                    Valor::Bool(true) => return Ok(Valor::Bool(true)),
                    Valor::Bool(false) => {
                        let val_r = avaliar_expr(r, ctx)?;
                        return match val_r {
                            Valor::Bool(b) => Ok(Valor::Bool(b)),
                            outro => Err(ErroExecucao::new(
                                r.span,
                                format!("operador `or` espera Bool, encontrado {}", outro),
                            )),
                        };
                    }
                    outro => {
                        return Err(ErroExecucao::new(
                            l.span,
                            format!("operador `or` espera Bool, encontrado {}", outro),
                        ));
                    }
                }
            }

            let vl = avaliar_expr(l, ctx)?;
            let vr = avaliar_expr(r, ctx)?;

            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                    let nl = match vl {
                        Valor::Numero(n) => n,
                        outro => {
                            return Err(ErroExecucao::new(
                                l.span,
                                format!("operador aritmético espera Number, encontrado {}", outro),
                            ));
                        }
                    };
                    let nr = match vr {
                        Valor::Numero(n) => n,
                        outro => {
                            return Err(ErroExecucao::new(
                                r.span,
                                format!("operador aritmético espera Number, encontrado {}", outro),
                            ));
                        }
                    };
                    match op {
                        BinOp::Add => Ok(Valor::Numero(nl + nr)),
                        BinOp::Sub => Ok(Valor::Numero(nl - nr)),
                        BinOp::Mul => Ok(Valor::Numero(nl * nr)),
                        BinOp::Div => {
                            if nr == 0.0 {
                                Err(ErroExecucao::new(r.span, "divisão por zero"))
                            } else {
                                Ok(Valor::Numero(nl / nr))
                            }
                        }
                        BinOp::Mod => {
                            if nr == 0.0 {
                                Err(ErroExecucao::new(r.span, "divisão por zero no operador `%`"))
                            } else {
                                Ok(Valor::Numero(nl % nr))
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                BinOp::Gt | BinOp::Gte | BinOp::Lt | BinOp::Lte => {
                    // Option ausente avalia para false nas comparações relacionais
                    if vl == Valor::Ausente || vr == Valor::Ausente {
                        return Ok(Valor::Bool(false));
                    }
                    let nl = match vl {
                        Valor::Numero(n) => n,
                        outro => {
                            return Err(ErroExecucao::new(
                                l.span,
                                format!("comparação relacional espera Number, encontrado {}", outro),
                            ));
                        }
                    };
                    let nr = match vr {
                        Valor::Numero(n) => n,
                        outro => {
                            return Err(ErroExecucao::new(
                                r.span,
                                format!("comparação relacional espera Number, encontrado {}", outro),
                            ));
                        }
                    };
                    let res = match op {
                        BinOp::Gt => nl > nr,
                        BinOp::Gte => nl >= nr,
                        BinOp::Lt => nl < nr,
                        BinOp::Lte => nl <= nr,
                        _ => unreachable!(),
                    };
                    Ok(Valor::Bool(res))
                }
                BinOp::Eq => Ok(Valor::Bool(comparar_igualdade(&vl, &vr))),
                BinOp::Neq => Ok(Valor::Bool(!comparar_igualdade(&vl, &vr))),
                BinOp::And | BinOp::Or => unreachable!(),
            }
        }
    }
}

fn comparar_igualdade(a: &Valor, b: &Valor) -> bool {
    match (a, b) {
        (Valor::Numero(x), Valor::Numero(y)) => (x - y).abs() < f64::EPSILON,
        (Valor::Texto(x), Valor::Texto(y)) => x == y,
        (Valor::Bool(x), Valor::Bool(y)) => x == y,
        (Valor::No(x), Valor::Texto(y)) | (Valor::Texto(y), Valor::No(x)) => x == y,
        (Valor::No(x), Valor::No(y)) => x == y,
        (Valor::Ausente, Valor::Ausente) => true,
        (Valor::Ausente, _) | (_, Valor::Ausente) => false,
        _ => false,
    }
}

// ========== Algoritmo Dijkstra para short_dist

#[derive(Copy, Clone, PartialEq)]
struct EstadoDijkstra<'a> {
    custo: f64,
    no: &'a str,
}

impl Eq for EstadoDijkstra<'_> {}

impl Ord for EstadoDijkstra<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .custo
            .partial_cmp(&self.custo)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for EstadoDijkstra<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn calcular_short_dist(
    grafo: &GrafoRuntime,
    origem: &str,
    destino: &str,
    span: Span,
) -> Result<f64, ErroExecucao> {
    if !grafo.nos.contains_key(origem) {
        return Err(ErroExecucao::new(
            span,
            format!("nó `{}` não existe no grafo `{}`", origem, grafo.nome),
        ));
    }
    if !grafo.nos.contains_key(destino) {
        return Err(ErroExecucao::new(
            span,
            format!("nó `{}` não existe no grafo `{}`", destino, grafo.nome),
        ));
    }

    if origem == destino {
        return Ok(0.0);
    }

    // Monta lista de adjacência
    let mut adj: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
    for no in grafo.nos.keys() {
        adj.insert(no.as_str(), Vec::new());
    }

    for a in &grafo.arestas {
        adj.entry(&a.origem).or_default().push((&a.destino, a.peso));
        if a.tipo == TipoAresta::NaoDirecionada {
            adj.entry(&a.destino).or_default().push((&a.origem, a.peso));
        }
    }

    let mut distancias: HashMap<&str, f64> = HashMap::new();
    distancias.insert(origem, 0.0);

    let mut heap = BinaryHeap::new();
    heap.push(EstadoDijkstra {
        custo: 0.0,
        no: origem,
    });

    while let Some(EstadoDijkstra { custo, no }) = heap.pop() {
        if no == destino {
            return Ok(custo);
        }

        if let Some(&d) = distancias.get(no) {
            if custo > d {
                continue;
            }
        }

        if let Some(vizinhos) = adj.get(no) {
            for &(vizinho, peso) in vizinhos {
                let proximo_custo = custo + peso;
                let menor_atual = distancias.get(vizinho).copied().unwrap_or(f64::INFINITY);

                if proximo_custo < menor_atual {
                    distancias.insert(vizinho, proximo_custo);
                    heap.push(EstadoDijkstra {
                        custo: proximo_custo,
                        no: vizinho,
                    });
                }
            }
        }
    }

    Err(ErroExecucao::new(
        span,
        format!(
            "nós `{}` e `{}` não estão conectados no grafo `{}`",
            origem, destino, grafo.nome
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interpretar(fonte: &str) -> Result<(Interpretador, Vec<ResultadoConsulta>), String> {
        let programa = crate::grammar::ProgramaParser::new()
            .parse(crate::lexer::Lexer::new(fonte))
            .map_err(|e| format!("Erro de parsing: {:?}", e))?;

        crate::checker::checar(&programa).map_err(|erros| {
            format!("Erros semânticos: {:?}", erros.iter().map(|e| &e.msg).collect::<Vec<_>>())
        })?;

        let mut interp = Interpretador::new();
        let res = interp.executar(&programa).map_err(|e| e.msg)?;
        Ok((interp, res))
    }

    #[test]
    fn interpreta_grafo_simples_e_short_dist() {
        let prog = r#"
            Graph G {
                Node = {a, b, c, d}
                Edge = {
                    a -> b : 2,
                    b -> c : 3,
                    c -> d : 4
                }
                Edge = a -> d : 15
            }
            G.short_dist(a, d)
        "#;
        let (interp, res) = interpretar(prog).expect("deveria executar com sucesso");
        assert!(interp.grafos().contains_key("G"));
        assert_eq!(interp.grafos()["G"].nos.len(), 4);
        assert_eq!(interp.grafos()["G"].arestas.len(), 4);
        assert_eq!(res.len(), 1);
        match &res[0] {
            ResultadoConsulta::Distancia { distancia, .. } => assert_eq!(*distancia, 9.0),
            _ => panic!("esperava ResultadoConsulta::Distancia"),
        }
    }

    #[test]
    fn interpreta_aresta_nao_direcionada() {
        let prog = r#"
            Graph G {
                Node = {x, y}
                Edge = x -- y : 7
            }
            G.short_dist(y, x)
        "#;
        let (_, res) = interpretar(prog).expect("deveria executar com sucesso");
        match &res[0] {
            ResultadoConsulta::Distancia { distancia, .. } => assert_eq!(*distancia, 7.0),
            _ => panic!("esperava ResultadoConsulta::Distancia"),
        }
    }

    #[test]
    fn erro_execucao_nos_desconectados() {
        let prog = r#"
            Graph G {
                Node = {a, b, c}
                Edge = a -> b : 5
            }
            G.short_dist(a, c)
        "#;
        let erro = interpretar(prog).expect_err("deveria falhar com nós desconectados");
        assert!(erro.contains("não estão conectados"));
    }

    #[test]
    fn interpreta_grafo_merge_com_pontes() {
        let prog = r#"
            Graph G1 {
                Node = {a, b}
                Edge = a -> b : 3
            }
            Graph G2 {
                Node = {c, d}
                Edge = c -- d : 2
            }
            Graph G3 = G1.merge(G2, G1.b -> G2.c : 4, G2.d -> G1.a)
            G3.short_dist(a, d)
        "#;
        let (interp, res) = interpretar(prog).expect("merge deveria executar");
        assert!(interp.grafos().contains_key("G3"));
        assert_eq!(interp.grafos()["G3"].nos.len(), 4);
        assert_eq!(interp.grafos()["G3"].arestas.len(), 4);
        match &res[0] {
            ResultadoConsulta::Distancia { distancia, .. } => assert_eq!(*distancia, 9.0),
            _ => panic!("esperava ResultadoConsulta::Distancia"),
        }
    }

    #[test]
    fn interpreta_filter_com_atributos_e_tipos() {
        let prog = r#"
            Graph rede {
                NodeType Pessoa { nome: String, idade: Number, ativo: Option<Bool> }
                Node = ana   : Pessoa { nome: "Ana", idade: 20 }
                Node = bia   : Pessoa { idade: 31, nome: "Bia", ativo: True }
                Node = carla : Pessoa { nome: "Carla", idade: 17, ativo: False }

                Edge = {
                    ana -> bia : 5,
                    bia -- carla : 2
                }
                Edge = carla -> ana
            }
            rede.short_dist(ana, carla)
            rede.filter(for e in rede | e.weight >= 2 and not e.destination == "carla")
            rede.filter(for e in rede | e.origin.idade > 18 and e.type == "->")
            rede.filter(for e in rede | e.destination.ativo == True)
        "#;
        let (_, res) = interpretar(prog).expect("deveria executar com sucesso");
        assert_eq!(res.len(), 4);

        // 1. short_dist
        match &res[0] {
            ResultadoConsulta::Distancia { distancia, .. } => assert_eq!(*distancia, 7.0),
            _ => panic!("esperava short_dist = 7"),
        }

        // 2. filter 1
        match &res[1] {
            ResultadoConsulta::Filtro { arestas, .. } => {
                assert_eq!(arestas.len(), 1);
                assert_eq!(arestas[0].origem, "ana");
                assert_eq!(arestas[0].destino, "bia");
                assert_eq!(arestas[0].peso, 5.0);
            }
            _ => panic!("esperava Filtro"),
        }

        // 3. filter 2
        match &res[2] {
            ResultadoConsulta::Filtro { arestas, .. } => {
                assert_eq!(arestas.len(), 1);
                assert_eq!(arestas[0].origem, "ana");
                assert_eq!(arestas[0].destino, "bia");
            }
            _ => panic!("esperava Filtro"),
        }

        // 4. filter 3
        match &res[3] {
            ResultadoConsulta::Filtro { arestas, .. } => {
                assert_eq!(arestas.len(), 1);
                assert_eq!(arestas[0].origem, "ana");
                assert_eq!(arestas[0].destino, "bia");
            }
            _ => panic!("esperava Filtro"),
        }
    }

    #[test]
    fn interpreta_consultas_encadeadas_filter_short_dist() {
        let prog = r#"
            Graph G {
                Node = {a, b, c}
                Edge = {
                    a -> b : 10,
                    b -> c : 10,
                    a -> c : 100
                }
            }
            G.filter(for e in G | e.weight > 15).short_dist(a, c)
        "#;
        let (_, res) = interpretar(prog).expect("deveria executar chamada encadeada");
        assert_eq!(res.len(), 1);
        match &res[0] {
            ResultadoConsulta::Distancia { distancia, .. } => assert_eq!(*distancia, 100.0),
            _ => panic!("esperava short_dist no grafo filtrado"),
        }
    }

    #[test]
    fn interpreta_filtro_vazio() {
        let prog = r#"
            Graph G {
                Node = {a, b}
                Edge = a -> b : 1
            }
            G.filter(for e in G | e.weight > 10)
        "#;
        let (_, res) = interpretar(prog).expect("deveria executar com sucesso");
        match &res[0] {
            ResultadoConsulta::Filtro { arestas, .. } => assert!(arestas.is_empty()),
            _ => panic!("esperava Filtro vazio"),
        }
    }

    #[test]
    fn detecta_avisos_variaveis_nao_utilizadas() {
        let prog = r#"
            Graph G {
                Node = {a, b, isolado}
                Edge = a -> b
            }
            Graph H {
                Node = x
            }
            G.filter(for e in G | True)
        "#;
        let programa = crate::grammar::ProgramaParser::new()
            .parse(crate::lexer::Lexer::new(prog))
            .unwrap();
        let avs = avisos(&programa);
        let msgs: Vec<String> = avs.into_iter().map(|a| a.msg).collect();
        assert!(msgs.iter().any(|m| m.contains("grafo `H` declarado mas não utilizado")));
        assert!(msgs.iter().any(|m| m.contains("nó `isolado` no grafo `G` declarado mas não utilizado")));
        assert!(msgs.iter().any(|m| m.contains("variável `e` no filtro de `G` declarada mas não utilizada")));
    }
}
