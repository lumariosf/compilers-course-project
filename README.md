
# GRAPH language

GRAPH é uma linguagem específica de domínio (DSL) para definição e consulta de grafos. Este repositório contém a especificação do Parser de GRAPH como parte do trabalho feito para a matéria de Compiladores do semestre 2026.2 pelo grupo 5 (Giovanna Marques Mafra, Heitor Cordeiro Holanda, Luma Rios Ferreira e Mariana Sousa Freire).

Para criar o código do Parser de GRAPH, utilizamos o parser generator [Larlpop](https://github.com/lalrpop/lalrpop), por ser um framework que tem usabilidade como seu principal objetivo e por gerar o Parser diretamente em Rust.

## Diretórios

```text
.
├── examples/
│   ├── grafo1.graph ... grafo9.graph
│   └── test.graph
├── src/
│   ├── ast.rs
│   ├── grammar.lalrpop
│   ├── grammar.rs
│   ├── lexer.rs
│   └── main.rs
├── tests/
│   ├── integration/
│   │   ├── examples.test
│   │   ├── ast-values.graph
│   │   ├── ast-values.test
│   │   ├── graph-features.graph
│   │   ├── graph-features.test
│   │   ├── query-filter-logic.graph
│   │   ├── query-merge-multiple-bridges.graph
│   │   ├── query-merge-one-bridge.graph
│   │   ├── query-merge-one-undirected-bridge.graph
│   │   ├── query-short-dist-variants.graph
│   │   ├── query-variants.test
│   │   ├── queries.graph
│   │   ├── queries.test
│   │   ├── snapshot-short-dist.graph
│   │   ├── snapshot-two-nodes.graph
│   │   ├── snapshot-weighted-edge.graph
│   │   └── snapshots.test
│   ├── lexer/
│   │   ├── boolean-case.graph
│   │   ├── boundary-cases.test
│   │   ├── digits-only-identifier.graph
│   │   ├── identifiers.graph
│   │   ├── identifiers.test
│   │   ├── invalid-token.graph
│   │   ├── keyword-as-identifier.graph
│   │   ├── number-leading-dot.graph
│   │   ├── number-trailing-dot.graph
│   │   └── string-unclosed-quote.graph
│   └── parser/
│       ├── additional-cases.test
│       ├── edge-missing-destination.graph
│       ├── filter-double-pipe.graph
│       ├── graph-missing-name.graph
│       ├── identifier-leading-digit.graph
│       ├── identifier-leading-underscore.graph
│       ├── merge-no-bridges.graph
│       ├── merge-unqualified-bridge.graph
│       ├── malformed.graph
│       ├── negative-edge-weight.graph
│       ├── nodetype-trailing-comma.graph
│       ├── program-boundaries.graph
│       └── program-boundaries.test
├── .gitignore
├── build.rs
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## Testes automatizados

### Objetivo e escopo

A suíte automatizada verifica regressões na análise léxica, na análise sintática e na interface de linha de comando do compilador GRAPH. Os casos exercitam entradas válidas e inválidas e comparam a saída observada com as expectativas definidas para cada comando.

O executável atual lê um arquivo GRAPH, analisa o conteúdo e, quando aceito, imprime a AST formatada em `stdout`. Erros de leitura ou parsing são escritos em `stderr`. A suíte verifica esse comportamento, incluindo o código de saída atual: a CLI retorna `0` também nos casos de erro. Os testes não executam consultas nem verificam regras semânticas; não há uma categoria `semantic/` porque essa etapa não está implementada na CLI atual.

### ShellTestRunner

[ShellTestRunner](https://github.com/simonmichael/shelltestrunner), executado pelo comando `shelltest`, é uma ferramenta externa para testar programas e comandos de terminal. Ele foi escolhido porque o compilador é usado como CLI e seus testes precisam verificar comandos, `stdout`, `stderr` e status de saída sem alterar o código de produção.

Cada arquivo `.test` descreve um ou mais casos usando o formato 3 do ShellTestRunner:

| Elemento | Significado nesta suíte |
| --- | --- |
| Arquivo `.test` | Especificação de casos e expectativas que o `shelltest` executa. |
| `$ comando` | Comando de shell a executar. Nos casos GRAPH, chama `cargo run --quiet --` com um arquivo de entrada. |
| Entrada | O programa recebe o caminho do arquivo `.graph` como argumento. Os testes atuais não enviam o código GRAPH por `stdin`. |
| `stdout` | Saída padrão do comando. Nos testes válidos, alguns comandos usam `sed` e `grep` para selecionar partes da AST antes da comparação. |
| `>2` | Inicia a expectativa de `stderr`; pode ser texto literal ou uma expressão regular entre barras, como `>2 /.*Erro de parsing:.*/`. |
| `>= código` | Código de saída esperado. Os testes atuais especificam `>= 0`, inclusive para erros reportados pela CLI. |
| Saída esperada | Linhas escritas depois do comando (ou depois de `>`) até o próximo marcador. Para uma expectativa por regex, usa-se o marcador de stream seguido por `/expressão/`. |
| PASS / FAIL | O runner compara os streams e o status obtidos com as expectativas. Casos aprovados aparecem como `[OK]`; casos reprovados como `[Failed]`. |

Quando um stream não tem conteúdo esperado, o teste o deixa vazio. Por exemplo, um teste que começa suas expectativas com `>2` espera `stdout` vazio e verifica `stderr`.

Não há configuração específica do ShellTestRunner no `Cargo.toml` nem um arquivo separado do runner. Os comandos, entradas e expectativas estão nos próprios `.test`; os arquivos `.graph` ao lado deles são fixtures usadas como entrada.

### Estrutura e cobertura

```text
tests/
├── integration/
│   ├── examples.test
│   ├── ast-values.graph
│   ├── ast-values.test
│   ├── graph-features.graph
│   ├── graph-features.test
│   ├── query-filter-logic.graph
│   ├── query-merge-multiple-bridges.graph
│   ├── query-merge-one-bridge.graph
│   ├── query-merge-one-undirected-bridge.graph
│   ├── query-short-dist-variants.graph
│   ├── query-variants.test
│   ├── queries.graph
│   ├── queries.test
│   ├── snapshot-short-dist.graph
│   ├── snapshot-two-nodes.graph
│   ├── snapshot-weighted-edge.graph
│   └── snapshots.test
├── lexer/
│   ├── boolean-case.graph
│   ├── boundary-cases.test
│   ├── digits-only-identifier.graph
│   ├── identifiers.graph
│   ├── identifiers.test
│   ├── invalid-token.graph
│   ├── keyword-as-identifier.graph
│   ├── number-leading-dot.graph
│   ├── number-trailing-dot.graph
│   └── string-unclosed-quote.graph
└── parser/
    ├── additional-cases.test
    ├── edge-missing-destination.graph
    ├── filter-double-pipe.graph
    ├── graph-missing-name.graph
    ├── identifier-leading-digit.graph
    ├── identifier-leading-underscore.graph
    ├── merge-no-bridges.graph
    ├── merge-unqualified-bridge.graph
    ├── malformed.graph
    ├── negative-edge-weight.graph
    ├── nodetype-trailing-comma.graph
    ├── program-boundaries.graph
    └── program-boundaries.test
```

- `tests/lexer/`: verifica identificadores e tokens inválidos, aspas de string não fechadas, maiúsculas/minúsculas de booleanos, formatos `.5` e `5.`, palavras reservadas usadas como identificadores e nomes compostos apenas por dígitos. Como os testes chamam a CLI, também exercitam a interação com o parser.
- `tests/parser/`: cobre programa vazio, sintaxe incompleta, aresta sem destino, declaração `Graph` sem nome, identificadores que começam por `_` ou dígito, vírgula final em `NodeType`, formas de `merge` e uso de `||` em filtro. Alguns casos registram aceitações e rejeições reais da gramática que não coincidem com a intenção/documentação; consulte “Limitações conhecidas”.
- `tests/integration/`: combina construções e cobre os exemplos existentes em `examples/`; verifica valores selecionados da AST (nomes, atributos, booleanos, pesos e tipos de aresta), variações de `short_dist` e `filter`, tentativas de `merge` com pontes, e erros da CLI. `snapshots.test` compara a AST completa em três programas pequenos; outros testes usam `grep` para selecionar trechos.

O arquivo `examples.test` executa `examples/grafo1.graph` até `examples/grafo9.graph` e `examples/test.graph`. Os exemplos de `grafo1` a `grafo5` e `test.graph` são aceitos pelo parser; `grafo6` a `grafo9` são casos inválidos que verificam os diagnósticos atuais. A aceitação de uma chamada de consulta nesses testes significa que sua sintaxe foi analisada e representada na AST, não que a consulta tenha sido executada.

### Limitações conhecidas

- A CLI retorna status `0` mesmo quando encontra erro de leitura ou parsing. Os testes mantêm `>= 0` e validam os diagnósticos em `stderr`.
- Não há testes semânticos nem testes de execução de consultas: a implementação testada aqui produz a AST e não executa `short_dist`, `filter` ou `merge`.
- Os testes com `grep` verificam apenas a presença e os valores selecionados pela expressão de busca, não a AST inteira. As três entradas de `snapshots.test` são exceções e comparam toda a AST formatada.
- A gramática aceita vírgula final na lista de campos de `NodeType` e aceita a chamada `merge` sem pontes. A forma `.merge(Grafo, pontes...)` com pontes qualificadas é rejeitada pela gramática atual, assim como `||` em uma expressão de filtro. Os testes registram esses resultados reais sem alterar o compilador.
- O token `True` produz um literal booleano; `true` minúsculo é reconhecido como identificador e pode aparecer na AST de um atributo, pois não há verificação semântica de tipos nessa etapa.

### Formato e exemplo

Este caso está em `tests/integration/examples.test`:

```text
$ cargo run --quiet -- examples/grafo1.graph | sed -E 's/^[[:space:]]*//' | grep -E '^(Grafo|Consulta)\('
Grafo(
Consulta(
>2
>= 0
```

Passo a passo:

1. `$` inicia o comando. `cargo run --quiet -- examples/grafo1.graph` executa o compilador usando o exemplo existente como arquivo de entrada. O restante do comando remove a indentação da AST e seleciona os itens `Grafo(` e `Consulta(`.
2. As duas linhas seguintes são o `stdout` esperado depois dos filtros. Assim, o caso confirma que esse exemplo gera uma declaração de grafo e uma consulta na AST.
3. `>2` inicia a expectativa de `stderr`, que fica vazia neste caso válido.
4. `>= 0` exige status de saída zero.

Um exemplo de erro real, em `tests/lexer/identifiers.test`, é:

```text
$ cargo run --quiet -- tests/lexer/invalid-token.graph
>2 /.*Erro de parsing:.*Token inválido na posição.*/
>= 0
```

O fixture contém `@`, que não é um token válido. A regex verifica a mensagem enviada a `stderr`; como não há linhas de saída esperada antes de `>2`, espera-se `stdout` vazio. O status `0` registra o comportamento atual da CLI ao reportar esse erro.

### Instalação e compilação

O projeto usa Rust e Cargo. No ambiente Linux atual, compatível com Debian/Ubuntu, instale o ShellTestRunner com:

```sh
sudo apt update
sudo apt install shelltestrunner
```

Confira a instalação com `shelltest --version`. A suíte foi verificada com ShellTestRunner 1.9. O runner é uma ferramenta do sistema, não uma dependência Cargo do projeto.

Compile o compilador a partir da raiz do repositório:

```sh
cargo build
```

O executável de debug gerado é `target/debug/compilers-course-project`. Para executar diretamente um programa GRAPH:

```sh
./target/debug/compilers-course-project examples/grafo1.graph
```

Também é possível executá-lo através do Cargo, como fazem os testes:

```sh
cargo run --quiet -- examples/grafo1.graph
```

### Executar os testes

Execute os comandos a partir da raiz do repositório, onde os caminhos relativos dos testes são válidos.

Para compilar e depois executar todos os testes:

```sh
cargo build && shelltest tests
```

O comando direto para executar toda a suíte é:

```sh
shelltest tests
```

Para executar uma categoria ou um arquivo individual:

```sh
shelltest tests/lexer
shelltest tests/integration
shelltest tests/integration/examples.test
```

O ShellTestRunner procura arquivos `.test` recursivamente quando recebe um diretório. Cada comando de teste usa `cargo run --quiet --`, que compila ou reutiliza a compilação conforme necessário; por isso `cargo build` é uma etapa preparatória opcional, mas útil para confirmar a compilação antes da suíte.

### Interpretar os resultados

O ShellTestRunner lista o resultado de cada caso e apresenta um resumo com `Passed`, `Failed` e `Total`. `[OK]` indica que as saídas e o status correspondem às expectativas; `[Failed]` indica divergência. A saída da falha informa valores como `Expected stdout` e `Got stdout` (ou equivalentes para `stderr` e status), permitindo localizar o que mudou.

Uma mensagem `Erro de parsing: ...` ou `Erro ao ler arquivo: ...` emitida pelo compilador pode ser o resultado esperado de um caso inválido e, nesse caso, o teste passa. O teste falha se a mensagem não corresponder à expectativa. Já uma mensagem de erro de compilação Rust, um comando `shelltest` não encontrado, um caminho de fixture incorreto ou um erro de sintaxe no `.test` aponta para uma falha de build, ambiente, execução ou configuração do teste, e não para o diagnóstico GRAPH esperado.

Na implementação atual, erros da CLI não retornam status diferente de zero. Por isso os casos inválidos esperam `>= 0` e verificam o texto em `stderr`; esses testes documentam o comportamento existente, sem alterá-lo.

### Adicionar um teste

1. Crie ou escolha a categoria correspondente em `tests/`: `lexer/`, `parser/` ou `integration/`. Use integração quando o caso combinar funcionalidades ou usar um dos exemplos gerais.
2. Crie um arquivo `.test` nessa pasta. Se o compilador precisar de uma entrada GRAPH, crie também um fixture `.graph`; a CLI recebe esse arquivo como argumento.
3. Adicione um comando iniciado por `$`. Declare as linhas esperadas de `stdout`; use `>2` para verificar `stderr` e `>=` para o código de saída. Para mensagens variáveis, use uma regex delimitada por `/`.
4. Execute o arquivo específico, por exemplo `shelltest tests/parser/novo-caso.test`, ou toda a categoria, por exemplo `shelltest tests/parser`.
5. Confirme `[OK]` no caso e o resumo sem falhas. Em `[Failed]`, compare a saída esperada com a saída obtida antes de decidir se a expectativa ou o comportamento do compilador deve mudar.

### Boas práticas

- Faça cada caso verificar um comportamento específico e mantenha casos válidos, inválidos, de erro e de borda.
- Organize `.test` e fixtures pela funcionalidade que exercitam e use caminhos relativos à raiz do projeto.
- Não altere o código de produção apenas para fazer um teste passar; primeiro confirme o comportamento definido pela implementação e pela linguagem.
- Atualize as expectativas quando o comportamento esperado do compilador mudar e execute novamente a suíte completa.

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