#!/usr/bin/env sh
# Instala a extensão GRAPH no VS Code local (copia a pasta para ~/.vscode/extensions).
# Depois de rodar, recarregue o VS Code: Ctrl+Shift+P -> "Developer: Reload Window".
set -e

SRC="$(cd "$(dirname "$0")" && pwd)"
DEST="${VSCODE_EXTENSIONS:-$HOME/.vscode/extensions}/grupo5-compiladores.graph-language-1.0.0"

rm -rf "$DEST"
mkdir -p "$DEST"
cp -r "$SRC/package.json" "$SRC/language-configuration.json" \
      "$SRC/icons" "$SRC/syntaxes" "$SRC/snippets" "$DEST/"

echo "Extensão GRAPH instalada em: $DEST"
echo "Recarregue o VS Code (Ctrl+Shift+P -> Developer: Reload Window)."
