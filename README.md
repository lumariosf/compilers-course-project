
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
    │   ├── grammar.lalrpop
    |   ├── grammar.rs
    │   ├── lexer.rs
    │   └── main.rs
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

```
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