# Suíte de testes

O comando canônico é `make test`, documentado no README da raiz. Ele combina os testes unitários Rust com a comparação de todos os pares `.graph` + `.out` em `fixtures/` e dos exemplos originais em `examples/`.

`run-golden.sh --bless` regenera snapshots executando o binário local. Revise cada saída gerada manualmente; a execução normal do runner é somente leitura e apresenta diffs para snapshots divergentes.

Os arquivos `.test` legados em `cases/` mantêm expectativas de uma versão anterior da CLI e não são executados pelo comando canônico. Os snapshots `.out` cobrem a CLI atual, incluindo a AST impressa para programas semanticamente válidos.
