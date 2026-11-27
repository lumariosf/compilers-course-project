# <img src="icons/graph.svg" width="28" align="top"> GRAPH para VS Code

Extensão local que dá identidade visual aos arquivos `.graph`:

- **Ícone** próprio no explorador de arquivos e nas abas;
- **Destaque de sintaxe**: palavras-chave (`Graph`, `Node`, `Edge`, `NodeType`, `let`, `func`, `if/then/else`, `for/in`), consultas (`short_dist`, `short_path`, `connected`, `filter`, `merge`), tipos (`Number`, `String`, `Bool`, `Option`), operadores de aresta (`->`, `--`), campos de aresta (`weight`, `origin`, `destination`, `type`), strings, números, booleanos e comentários `//`;
- **Editor**: `Ctrl+/` comenta a linha, fecha `{}`, `()` e `""` automaticamente;
- **Snippets**: digite `graph`, `nodetype`, `func`, `let`, `print`, `if`, `filter`, `short_dist`, `short_path`, `connected` ou `merge` e aperte `Tab`.

## Instalação

```bash
./editor/vscode-graph/install.sh
```

Depois recarregue o VS Code (`Ctrl+Shift+P` → **Developer: Reload Window**) e abra qualquer arquivo de `examples/`.

> As cores seguem o tema do VS Code em uso. O ícone aparece com o tema de ícones padrão (Seti); temas de ícones de terceiros podem sobrescrevê-lo.

Para desinstalar: `rm -rf ~/.vscode/extensions/grupo5-compiladores.graph-language-*`.
