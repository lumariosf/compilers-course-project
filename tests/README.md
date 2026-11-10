# Testes

Os casos usam ShellTestRunner (`shelltest`, formato 3). Execute os comandos a partir da raiz do repositório.

```text
tests/
├── README.md
├── cases/{integration,lexer,parser}/*.test
└── fixtures/
    ├── examples/ (grafo1.graph ... grafo9.graph, test.graph)
    ├── integration/{features,queries,snapshots}/*.graph
    ├── lexer/*.graph
    └── parser/*.graph
```

`cases/` contém os `.test` com comandos e expectativas. `fixtures/` contém as entradas `.graph`, incluindo os exemplos antes mantidos na raiz.

Todos os testes:
```sh
shelltest tests
```
Somente lexer:
```sh
shelltest tests/cases/lexer
```
Somente os casos dos exemplos:
```sh
shelltest tests/cases/integration/examples.test
```
