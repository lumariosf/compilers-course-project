# Relatório de Alterações e Testes

## Para que serve

Este documento explica a suíte de testes do projeto GRAPH. Ela verifica as etapas pelas quais um programa passa: leitura dos tokens, análise da gramática, verificação semântica e execução das consultas.

## Organização das pastas

```text
examples/                         Exemplos originais do projeto; não foram alterados.
src/                              Implementação do lexer, parser, checker e interpretador.
tests/
├── fixtures/
│   ├── examples/                 Cópias de exemplos menores usados por testes existentes.
│   ├── integration/
│   │   ├── features/             Combinações de recursos da linguagem.
│   │   ├── queries/              Consultas, filtros e variações de sintaxe.
│   │   └── snapshots/            Consultas com resultados esperados específicos.
│   ├── lexer/                    Tokens, identificadores e literais válidos ou inválidos.
│   ├── parser/                   Entradas aceitas ou rejeitadas pela gramática.
│   ├── runtime/                  Consultas executadas e erros durante a execução.
│   └── semantic/                 Erros de declaração, tipos, atributos e consultas.
├── expected/examples/            Saídas dos exemplos originais em examples/.
├── cases/                        Testes ShellTest antigos, fora do comando canônico.
└── run-golden.sh                 Executa entradas e compara seus arquivos .out.
```

Cada entrada `.graph` em `tests/fixtures/` tem um `.out` ao lado. Para não colocar arquivos de teste dentro da pasta original, os resultados dos 20 exemplos de `examples/` ficam em `tests/expected/examples/`.

## O que foi alterado

- A CLI agora mostra a AST quando o programa passa pela verificação semântica. Em seguida, mostra avisos e resultados das consultas, se houver.
- A gramática exige uma ou mais pontes na declaração de `merge`; sem ponte, a entrada é rejeitada durante o parsing.
- Os testes unitários de merge foram alinhados à regra e incluem uma verificação explícita da rejeição sem ponte.
- O README foi corrigido para documentar `Option<T>`, o campo `e.type`, a forma completa de uma consulta `filter` e a exigência de pontes no `merge`.
- Foram acrescentados casos de teste para erros semânticos e de execução, como tipos e declarações inválidos, expressões aninhadas e divisão por zero.
- Os 20 exemplos originais foram incluídos na suíte sem modificar seus arquivos.

## O que significa CLI

CLI é a sigla de *Command-Line Interface*, ou interface de linha de comando. É a forma de usar um programa digitando comandos no terminal, em vez de interagir com janelas e botões. Neste projeto, a CLI recebe o caminho de um arquivo `.graph`, por exemplo `cargo run -- examples/semantica_1_ok_completo.graph`, e mostra a AST, os avisos, os resultados ou os erros.

Na suíte, o runner chama essa mesma aplicação para cada arquivo de entrada. Assim, os testes verificam o comportamento que uma pessoa realmente observa ao executar o programa pelo terminal, incluindo mensagens e código de saída.

## Antes e depois

| Antes | Agora |
|---|---|
| O README dizia que programas válidos imprimiam a AST, mas a CLI não a mostrava. | A saída da CLI inclui a AST de programas aprovados pelo checker. |
| A gramática aceitava uma declaração de `merge` sem pontes. | É obrigatório informar pelo menos uma ponte. |
| Não havia um comando único comparando todas as saídas esperadas. | `make test` executa os testes unitários e os snapshots de integração. |
| Vários casos não tinham saída esperada registrada. | Cada caso golden tem uma expectativa `.out` versionada. |

## O que os testes cobrem

Os números abaixo são casos de integração com snapshots. Um único caso pode verificar mais de uma mensagem ou comportamento.

- **Gramática e léxico: 42 casos.** Verificam entradas aceitas e rejeitadas, tokens, identificadores, literais, limites de programa, expressões aninhadas e sintaxe de `merge`.
- **Verificação semântica: 15 casos.** Verificam nomes declarados, nós e arestas duplicados ou inexistentes, `NodeType`, tipos e atributos, regras de `merge` e tipos de expressões em filtros.
- **Interpretador e execução: 18 casos.** Verificam `short_dist`, caminhos desconectados, arestas direcionadas e bidirecionais, filtros, atributos, merges encadeados, resultados e divisões por zero.
- **Testes unitários Rust: 20 testes.** Verificam partes internas do checker, do interpretador e da resolução de tipos.

Assim, a suíte tem **75 casos golden** (20 exemplos originais e 55 fixtures) e **20 testes unitários**. GRAPH é uma DSL de grafos: não possui funções, retornos, escopo tradicional, recursão, laços nem condicionais. Por isso, esses recursos não aparecem como casos de teste.

## Como um caso é testado

1. O runner compila o projeto e localiza os arquivos `.graph` em `examples/` e `tests/fixtures/`.
2. Executa a aplicação para cada entrada e captura três informações: código de saída, stdout (saída normal) e stderr (avisos e erros).
3. Compara o resultado atual com o `.out` correspondente. Se houver diferença, apresenta um diff e marca o caso como `FAIL`; se forem iguais, marca `PASS`.

Por exemplo, um programa válido pode ter `exit=0`, AST em stdout e nenhum erro em stderr. Um programa semanticamente inválido normalmente tem `exit=1`, nenhuma execução de consulta e as mensagens do checker em stderr. Os snapshots preservam exatamente o comportamento da CLI.

## Como executar

Na raiz do projeto, o comando completo é:

```bash
make test
```

O projeto usa Rust, então `cargo test` é o comando próprio do Cargo para compilar e executar os testes unitários escritos em Rust. Porém, os testes de entrada e saída `.graph` + `.out` são executados por outro programa, `tests/run-golden.sh`. Rodar esses dois comandos separadamente seria possível, mas exigiria lembrar e digitar ambos.

O `Makefile` reúne essas etapas em uma tarefa chamada `test`. Por isso, `make test` executa primeiro `cargo test` e depois `tests/run-golden.sh`, na ordem necessária. Essa escolha oferece um comando curto, único e repetível para a pessoa que testa, para o grupo e para o professor; também reduz o risco de executar só uma parte da suíte. O Makefile não substitui Cargo nem muda como o compilador é construído: ele apenas coordena os comandos existentes.

Para executar apenas os snapshots, use `make golden`. Para rodar a aplicação normalmente sobre um programa, use Cargo, por exemplo `cargo run -- examples/semantica_1_ok_completo.graph`.

Para adicionar um caso, crie um `.graph` na pasta da fase que deseja testar e seu `.out` esperado. `./tests/run-golden.sh --bless` executa todos os casos e regenera os snapshots; depois, é necessário revisar as saídas antes de mantê-las. A execução normal de `make test` apenas compara os arquivos, sem atualizá-los.

## Perguntas que podem surgir

**Por que existem arquivos `.out`?**

Eles registram o resultado exato que a CLI deve produzir. Assim, é possível notar se uma alteração muda uma AST, um resultado de consulta, um aviso, uma mensagem de erro ou o código de saída.

**O teste verifica só se o programa compila?**

Não. Os testes unitários verificam componentes internos; os casos golden executam programas completos, desde o parsing até a verificação semântica e, quando válidos, a execução.

**Por que usar `make test` em vez de apenas `cargo test`?**

`cargo test` executa os testes unitários Rust, mas não executa sozinho o runner dos arquivos `.graph` e `.out`. O Makefile coordena as duas etapas com um só comando. Assim, `make test` é mais simples de lembrar e garante que a suíte completa seja executada, enquanto Cargo continua responsável por compilar e testar o código Rust.

**Por que algumas entradas de teste devem falhar?**

Porque a suíte também precisa confirmar que erros são detectados na fase correta. Uma entrada inválida não é uma falha do teste quando o `.out` registra a rejeição esperada.

**Por que não há testes de funções, recursão ou laços?**

Essas construções não fazem parte da gramática atual de GRAPH. Os testes cobrem os recursos implementados, sem atribuir à linguagem funcionalidades que ela não possui.

**Por que `examples/grafo5.graph` é rejeitado?**

Ele usa `G2.merge(G1)` como uma consulta. A sintaxe documentada atualmente declara o grafo resultante, por exemplo `Graph G3 = G1.merge(G2, G1.a -> G2.b)`. O exemplo foi preservado, e sua rejeição está registrada no snapshot.

**Os arquivos `.test` em `tests/cases/` também são executados?**

Não pelo comando canônico. Eles são testes ShellTest antigos, com expectativas incompatíveis com a CLI atual. A suíte mantida e documentada é executada por `make test`.

## Resultado registrado

Na última execução de `make test`, passaram os **20 testes unitários** e os **75 casos golden**.

### Roteiro curto para apresentação

“CLI significa interface de linha de comando: é usar o programa pelo terminal, informando um arquivo GRAPH. Organizamos testes para as etapas de léxico e gramática, verificação semântica e execução. Cada programa de teste tem uma saída esperada, comparada com a saída real. O Makefile reúne os testes unitários do Cargo e os testes dos arquivos `.graph` em um único comando, `make test`. A suíte tem 75 casos de integração e 20 testes unitários; todos passaram.”