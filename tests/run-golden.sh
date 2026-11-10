#!/usr/bin/env bash
set -u

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
binary="$root/target/debug/compilers-course-project"
mode=${1:-}

if [[ -n "$mode" && "$mode" != "--bless" ]]; then
    printf 'Uso: %s [--bless]\n' "$0" >&2
    exit 2
fi

cd "$root"
cargo build --quiet

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

mapfile -d '' fixtures < <(find "$root/tests/fixtures" -type f -name '*.graph' -print0 | sort -z)
mapfile -d '' examples < <(find "$root/examples" -maxdepth 1 -type f -name '*.graph' -print0 | sort -z)

declare -A counts=( [grammar]=0 [semantic]=0 [interpreter]=0 )
failed=0
total=0

for input in "${examples[@]}" "${fixtures[@]}"; do
    relative=${input#"$root/"}
    if [[ "$relative" == examples/* ]]; then
        expected="$root/tests/expected/examples/$(basename "${input%.graph}").out"
    else
        expected="${input%.graph}.out"
    fi

    case "$relative" in
        examples/semantica_*|tests/fixtures/semantic/*) category=semantic ;;
        examples/interpretador_*|tests/fixtures/runtime/*|tests/fixtures/integration/queries/*|tests/fixtures/integration/snapshots/*) category=interpreter ;;
        *) category=grammar ;;
    esac

    if "$binary" "$relative" >"$tmp/stdout" 2>"$tmp/stderr"; then
        status=0
    else
        status=$?
    fi

    {
        printf 'exit=%s\nstdout:\n' "$status"
        cat "$tmp/stdout"
        printf 'stderr:\n'
        cat "$tmp/stderr"
    } >"$tmp/actual"

    total=$((total + 1))
    counts[$category]=$((counts[$category] + 1))

    if [[ "$mode" == "--bless" ]]; then
        mkdir -p "$(dirname "$expected")"
        cp "$tmp/actual" "$expected"
        printf 'UPDATED %s\n' "$relative"
    elif [[ ! -f "$expected" ]]; then
        printf 'FAIL %s: missing snapshot %s\n' "$relative" "${expected#"$root/"}"
        failed=$((failed + 1))
    elif diff -u "$expected" "$tmp/actual"; then
        printf 'PASS %s\n' "$relative"
    else
        printf 'FAIL %s: output differs\n' "$relative"
        failed=$((failed + 1))
    fi
done

printf '\nGrammar: %s | Semantics: %s | Interpreter: %s | Total: %s\n' \
    "${counts[grammar]}" "${counts[semantic]}" "${counts[interpreter]}" "$total"

if [[ "$mode" != "--bless" && "$failed" -gt 0 ]]; then
    printf 'FAIL: %s of %s golden tests failed\n' "$failed" "$total" >&2
    exit 1
fi

if [[ "$mode" == "--bless" ]]; then
    printf 'Snapshots generated. Review the .out files before committing them.\n'
else
    printf 'PASS: all %s golden tests passed\n' "$total"
fi