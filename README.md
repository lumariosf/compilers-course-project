
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


```
### 5. Declarando arestas
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

### 6. Consultas 
Atualmente, existem três tipos de consultas permitidas em GRAPH:

#### short_dist
Essa consulta retorna a menor distância entre dois nós de um grafo. Para fazer a consulta, siga o exemplo abaixo, especificando o grafo e quais dos seus nós serão consultados. 

```bash
 myGraph.short_dist( A, B ) 
```

**A consulta retornará um erro se os nós não existirem no grafo especificado.**

Para grafos que só possuem arestas sem peso, a consulta retornará **0**. Se os nós não forem conectados, a consulta retornará **-1**.

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
Essa consulta permite a junção do grafo atual com outro grafo. Deve ser passado em seu argumento qual o grafo que você deseja mergir.

``` bash 
graphA.merge(graphB) 
```

A consulta retorna um **bool**, True se a junção foi bem sucedida ou False se a junção não foi possível.