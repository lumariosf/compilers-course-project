
# GRAPH language

GRAPH é uma linguagem específica de domínio (DSL) para definição e consulta de grafos. Este repositório contém a especificação do Parser de GRAPH como parte do trabalho feito para a matéria de Compiladores do semestre 2026.2 pelo grupo 5 (Giovanna Marques Mafra, Heitor Cordeiro Holanda, Luma Rios Ferreira e Mariana Sousa Freire).

Para criar o código do Parser de GRAPH, utilizamos o parser generator [Larlpop](https://github.com/lalrpop/lalrpop), por ser um framework que tem usabilidade como seu principal objetivo e por gerar o Parser diretamente em Rust.

## Diretórios

```bash
 compilers-course-project /
    ├── examples/
    │   └── grafo1.graph
    ├── src/
    │   ├── ast.rs
    │   ├── checker.rs
    │   ├── grammar.lalrpop
    |   ├── grammar.rs
    │   ├── interpreter.rs
    │   ├── lexer.rs
    │   ├── main.rs
    │   └── types.rs
    ├── .gitignore
    ├── build.rs
    ├── Cargo.lock
    ├── Cargo.toml
    └── README.md
```

## Documentação
Com GRAPH, você pode declarar um grafo, especificar seus nós e suas arestas, e fazer consultas sobre ele.

### 1. Identificadores

Os identificadores em GRAPH podem ser declarados utilizando caracteres alfanuméricos, com a exceção de caracteres especiais como $, @, #, %, &, etc. Todos os identificadores devem **começar** com e devem ter pelo menos uma letra do alfabeto, maiúscula ou minúscula.

**Exemplos corretos :** `Graph G123`,`Node = {user_12, user_24 }`, `Edge = { U00sEr -> user123_ }`

**Exemplos incorretos :** `Graph _rede`,`Node = {%12, 24 }`, `Edge = { u$u -> a@a }` 

### 2. Tipos primitivos em GRAPH
Os tipos primitivos suportados por GRAPH são **numbers**, **strings** e **bool**.  
- Strings: armazena uma sequência de caracteres entre aspas duplas. 
- Numbers: tipo padrão para todos os pesos em GRAPH, são do tipo f64 em Rust (tipo de ponto flutuante que ocupa até 8 bytes).
- Bool: assume dois valores lógicos, `True` ou  `False`.

### 3. Declarando um grafo
Para declarar um grafo, use a palavra reservada `Graph` junto a um identificador. Especifique as informações dos seus grafos dentro do bloco que segue após o identificador entre chaves.

```bash
    Graph G1 {
          Node = { a, d }
          Edge = { a->d , d->d }
        }

``` 
*O código acima mostra um programa em GRAPH que especifica o grafo G1, que possui os nós a e d, com arestas direcionadas de a para d e de d para ele mesmo.*

### 4. Declarando nós
Para declarar um nó, inclua o identificador do nó na lista definida pela palavra reservada `Node` , dentro do bloco do grafo. Um nó também pode ser declarado sozinho e nesse caso não há necessidade das chaves.

```bash
Graph rede_social {
    Node = {user1, user2}
    Node = user3
}
```

**!** *Grafos diferentes podem compartilhar identificadores de nós, mas não compartilham nós, vide exemplo abaixo.*
```bash
Graph G1 {
         Node = { a , d }
         Edge = { a->d ,d->d }
}

Graph G2 {
         Node = { a , d }
         Edge = { a->d ,d->d }
}
```

### 5. Tipos de nó e atributos
 
Nós podem carregar dados (atributos). Para isso, o grafo declara um **`NodeType`**, que descreve os campos e seus tipos, e cada nó declara qual tipo usa e os valores dos seus atributos.
 
```
Graph rede {
    NodeType Pessoa { nome: String, idade: Number, ativo: Option<Bool> }
 
    Node = ana : Pessoa { nome: "Ana", idade: 20 }
    Node = bia : Pessoa { nome: "Bia", idade: 31, ativo: True }
    Edge = ana -> bia
}
```
 
Regras:
 
- O `NodeType` é declarado **dentro do grafo** e deve aparecer **antes** de ser usado por um nó.
- Os tipos dos campos são `Number`, `String`, `Bool`.
- O valor de um atributo deve ser um literal (`20`, `-5`, `"Ana"`, `True`) do tipo declarado.
- Cada atributo informado deve existir no `NodeType`, e não pode ser repetido.
- Nós sem tipo (`Node = a`) continuam válidos e simplesmente não têm atributos.
Todas essas verificações acontecem **em tempo de compilação**, antes de qualquer consulta ser executada.


### 6. Declarando arestas
Para declarar uma aresta, utilize o operador `--` para arestas bidirecionais ou o operador `->` para arestas unidirecionais entre dois nós e inclua a aresta na lista definida pela palavra reservada `Edge`. Assim como nós, arestas também podem ser declaradas sozinhas, sem necessidade de chaves.

Arestas tem 4 atributos que podem ser acessados: 

`origin`: identifica o nó da esquerda do operador.

`destination`: identifica o nó da direita do operador.

`edge type`: define o tipo da aresta, bidirecional ou unidirecional.

`weight`: define o peso da aresta, default de 1.

**Toda aresta declara deve ser entre dois nós previamente declarados.**
```bash 
Graph G3 {
    Node = {a, b, c, d}
    Edge = {
        a -> b,
        b -> c,
        c -> d
    }
    Node = e
    Edge = a -> e
}
```

Arestas em GRAPH tem valor default de **1**. Para definir o peso de uma aresta, utilize `:` após declarar a aresta e declare o peso logo depois.

```bash 
    Node = {user123, user124}
    Edge = {
        user123 -> user124 : 10
    }

    Edge = user124 -> user123 : 2
```

### 7. Consultas 
Atualmente, existem três tipos de consultas permitidas em GRAPH:

#### short_dist
Essa consulta retorna a menor distância entre dois nós de um grafo. Para fazer a consulta, siga o exemplo abaixo, especificando o grafo e quais dos seus nós serão consultados. 

```bash
 myGraph.short_dist( A, B ) 
```

**A consulta é rejeitada em tempo de compilação se os nós não existirem no grafo especificado.**
 
A consulta retorna a soma dos pesos do caminho mais curto entre os dois nós.
 
**Se os nós não forem conectados, a consulta gera um erro de execução**,

---
#### filter
Essa consulta permite a filtragem dos atributos de um grafo baseado na especificação de um filtro. Observe o exemplo de um filtro abaixo, que retorna as arestas em myGraph com peso pelo menos 5 e que não tem como destino X:

```bash

filter(for e in myGraph | e.weight >= 5 and not e.destination == "X")

```

**Não confundir o símbolo | que separa a expressão que especifica o grafo com o símbolo de OU lógico || em outras linguagens**

*OU lógico em GRAPH é especificado pela palavra reservada **or***

Genericamente, siga o modelo `for {variable} in {source} | {condition}` para definir o filtro dentro do filter.

---
#### merge
 
O `merge` **cria um novo grafo** a partir de dois grafos existentes, que permanecem intactos. O resultado deve ser ligado a um nome com a declaração `Graph`. Para conectar os dois grafos, informe uma ou mais **pontes** depois do segundo grafo. Cada ponte é uma aresta cujas extremidades são **qualificadas** com o nome do grafo (`Grafo.nó`):
 
```
Graph G3 = G1.merge(G2, G1.a -> G2.b : 10, G2.c -> G1.k)
```
 
Regras:
 
- Cada ponte liga um nó de `G1` a um nó de `G2` (ou vice-versa). Uma ponte entre dois nós do mesmo grafo é inválida.
- Os nós de cada ponte devem existir nos respectivos grafos.
- As pontes aceitam `->` e `--` e um peso opcional (default 1), assim como as arestas comuns.
- **Se `G1` e `G2` tiverem um nó com o mesmo nome, o `merge` é rejeitado.**
- Se `G1` e `G2` declararem um `NodeType` com o mesmo nome, as duas definições devem ser **idênticas**; caso contrário, o `merge` é rejeitado.
- O grafo resultante contém os nós, as arestas e os `NodeType` dos dois grafos, mais as pontes.
- O grafo derivado só pode ser usado depois da linha em que foi declarado.
Qualquer violação dessas regras é um erro de compilação.

### 8. Verificação semântica

Depois do parser, o programa passa pelo **checker** (`src/checker.rs` e `src/types.rs`), que verifica o que a gramática sozinha não consegue: se os nomes usados foram declarados, se os tipos dos atributos batem com o `NodeType` e se as expressões do `filter` são bem tipadas.

O checker percorre o arquivo **uma única vez, na ordem em que foi escrito**, por isso tudo precisa ser declarado antes de ser usado. Ele **não para no primeiro erro**: todos os erros são listados de uma vez, com linha e coluna, e o compilador termina com código de saída 1.

```
exemplo.graph:3:47: erro: atributo `idade`: esperado Number, encontrado String
exemplo.graph:5:19: erro: nó `carlos` não declarado em `G` antes desta aresta
2 erro(s) semântico(s)
```

#### Exemplos

A pasta `examples/` tem programas que mostram o checker em ação. Os comentários `(ok)` e `(erro)` em cada arquivo indicam o que deve passar e onde cada erro deve aparecer.

| Arquivo | Resultado esperado |
|---------|--------------------|
| `semantica_1_ok_completo.graph` | válido: usa todos os recursos da linguagem |
| `semantica_2_ok_merge.graph` | válido: merge com pontes |
| `semantica_3_erro_nos_e_arestas.graph` | 4 erros: nós e arestas |
| `semantica_4_erro_atributos.graph` | 6 erros: `NodeType` e atributos |
| `semantica_5_erro_merge.graph` | 5 erros: merge e pontes |
| `semantica_6_erro_consultas.graph` | 10 erros: `short_dist` e `filter` |

Para rodar um exemplo e ver só os erros (o `> /dev/null` esconde a AST, que é impressa quando o programa é válido):

```bash
cargo run -q -- examples/semantica_6_erro_consultas.graph > /dev/null
echo $?   # 1 se houve erro, 0 se o programa é válido
```

Para rodar todos de uma vez:

```bash
for f in examples/semantica_*.graph; do echo "== $f"; cargo run -q -- "$f" > /dev/null; done
```

#### Erros detectados

**Grafos, nós e arestas**
- Grafo declarado mais de uma vez.
- Nó declarado mais de uma vez no mesmo grafo (inclusive `Node = a` seguido de `Node = a : T {...}`).
- Aresta que usa um nó não declarado **antes** dela.
- Aresta repetida (ver decisões abaixo).

**`NodeType` e atributos**
- `NodeType` declarado mais de uma vez no mesmo grafo, ou com campo repetido.
- Tipo de campo inválido: só `Number`, `String`, `Bool` e `Option<T>` (sem `Option<Option<T>>`).
- Nó que usa um `NodeType` inexistente ou declarado depois dele.
- Atributo que não existe no `NodeType`, ou informado duas vezes.
- Valor de atributo que não é literal (`idade: 1 + 2`).
- Valor de tipo diferente do declarado (`idade: "vinte"`). Um campo `Option<T>` aceita um literal de `T`.
- Campo obrigatório ausente: campos que não são `Option<T>` precisam ser informados.

**`merge`**: todas as regras da seção `merge`, mais:
- `G1.merge(G1)` (merge de um grafo com ele mesmo).
- Ponte repetida.

**Consultas**
- Grafo não declarado.
- `short_dist` com nó que não existe no grafo.
- `G.filter(for e in H | ...)` com `H` diferente de `G`.
- Condição do `filter` que não é `Bool`.

#### Tipagem das expressões do `filter`

| Expressão | Exige | Resultado |
|-----------|-------|-----------|
| `e.weight` | | `Number` |
| `e.origin`, `e.destination` | | nó (compara com `String` pelo nome) |
| `e.type` | comparado só com `"->"` ou `"--"` | `String` |
| `e.origin.campo` | **todos** os nós do grafo têm `campo`, com o mesmo tipo | tipo do campo |
| `not x` | `x: Bool` | `Bool` |
| `-x` | `x: Number` | `Number` |
| `x and y`, `x or y` | `Bool` | `Bool` |
| `+ - * / %` | `Number` | `Number` |
| `> >= < <=` | `Number` (aceita `Option<Number>`) | `Bool` |
| `== !=` | mesmo tipo, ignorando `Option`; ou nó com `String` | `Bool` |

Qualquer outro identificador além da variável do `for` é erro (para comparar com um nó, use o nome entre aspas: `e.destination == "X"`).

#### Decisões tomadas

| Tema | Decisão | Motivo |
|------|---------|--------|
| `in` do `filter` | `H` precisa ser o próprio grafo em `G.filter(for e in H \| ...)` | a gramática exige o nome; o checker garante a coerência |
| `origin` / `destination` | são o **nó**, não só o nome: `e.destination == "X"` e `e.origin.idade > 18` são válidos | permite filtrar pelos atributos dos nós com segurança de tipos |
| Tipo da aresta | acessado como `e.type`, do tipo `String` (`"->"` ou `"--"`) | `edge type` (com espaço) não é parseável |
| Arestas repetidas | são erro; `a -- b` vale como `a -> b` e `b -> a`, então `a -- b` com `b -- a` ou com `a -> b` também é erro. Laços (`a -> a`) são permitidos | evita duas arestas iguais com pesos diferentes no `short_dist` e no `filter` |
| `NodeType` no `merge` | "definições idênticas" = mesmos campos com os mesmos tipos; a ordem dos campos não importa | os campos são sempre acessados pelo nome |
| Valor de atributo | literal: número, texto, `True`/`False` ou número negativo (`-5`) | conforme a seção 5 |
| Nós sem caminho no `short_dist` | continua sendo erro de execução, não de compilação | conforme a seção 7 |

> **Pendente de atualização nas seções anteriores:** a seção 5 ainda não cita `Option<T>` na lista de tipos de campo; a seção 6 descreve o tipo da aresta como `edge type`, que no `filter` é `e.type`; e a seção 7 ainda não mostra o acesso a atributos de nós (`e.origin.idade`). Essas seções serão revisadas depois.

### 9. Interpretador e Execução

Após a verificação semântica, o programa é executado pelo interpretador (`src/interpreter.rs`).

- **Declarações e merge:** instanciam grafos em tempo de execução, preservando nós, tipos e arestas.
- **`short_dist`:** calcula a distância mínima usando o algoritmo de Dijkstra. Se os nós não forem conectados, gera erro de execução com linha e coluna.
- **`filter`:** avalia a condição para cada aresta e retorna as arestas que atendem ao predicado.
- **Encadeamento:** chamadas encadeadas (ex: `G.filter(...).short_dist(...)`) operam sequencialmente sobre o grafo filtrado anterior.

#### Exemplos do Interpretador

| Arquivo | Descrição e resultado esperado |
|---------|--------------------------------|
| `interpretador_1_dijkstra.graph` | `short_dist` em grafos direcionados e bidirecionais (`--`), caminho ótimo ponderado e auto-laço |
| `interpretador_2_filter_atributos.graph` | `filter` com atributos de nós (`idade`, `ativo`), campos `Option` ausentes e tipo de aresta (`e.type`) |
| `interpretador_3_merge_e_encadeamento.graph` | `GrafoMerge` com pontes ponderadas e encadeamento (`filter` -> `short_dist` e `filter` -> `filter`) |
| `interpretador_4_erro_desconectado.graph` | erro de execução quando nós não estão conectados no `short_dist` |

Para rodar todos os exemplos do interpretador:

```bash
for f in examples/interpretador_*.graph; do echo "== $f"; cargo run -q -- "$f"; done
```


