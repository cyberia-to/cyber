#!/usr/bin/env python3
"""fix-vocab.py — regenerate ~~~vocab sections in .model files from tokenizer.json

Usage: python3 analizer/fix-vocab.py ~/llm
       python3 analizer/fix-vocab.py ~/llm qwen3-0.6b-abl

Reads name.tokenizer.json alongside name.model, generates correct TOML
escaping for [tokens] and [merges], patches the .model file in-place.
"""

import json
import os
import sys


def toml_escape(s: str) -> str:
    """Escape a string for TOML double-quoted format."""
    result = []
    for c in s:
        if c == '\\':
            result.append('\\\\')
        elif c == '"':
            result.append('\\"')
        elif c == '\n':
            result.append('\\n')
        elif c == '\t':
            result.append('\\t')
        elif c == '\r':
            result.append('\\r')
        elif ord(c) < 0x20:
            result.append(f'\\u{ord(c):04X}')
        else:
            result.append(c)
    return ''.join(result)


def generate_vocab_toml(tokenizer_json_path: str) -> str:
    """Generate correct vocab.toml content from tokenizer.json."""
    with open(tokenizer_json_path) as f:
        tok = json.load(f)

    model = tok.get("model", {})
    vocab = model.get("vocab", {})
    merges_raw = model.get("merges", [])
    model_type = model.get("type", "BPE")

    lines = ["[tokens]"]

    if model_type == "BPE" and isinstance(vocab, dict):
        # BPE: vocab is {token: id}
        sorted_vocab = sorted(vocab.items(), key=lambda x: x[1])
        for token, id in sorted_vocab:
            lines.append(f'{id} = "{toml_escape(token)}"')
    elif isinstance(vocab, list):
        # Unigram/SentencePiece: vocab is [[token, score], ...]
        for i, item in enumerate(vocab):
            token = item[0] if isinstance(item, list) else str(item)
            lines.append(f'{i} = "{toml_escape(token)}"')
    else:
        # Unknown format
        return ""

    if merges_raw:
        lines.append("")
        lines.append("[merges]")
        for i, merge in enumerate(merges_raw):
            if isinstance(merge, list) and len(merge) == 2:
                a, b = merge
            elif isinstance(merge, str):
                parts = merge.split(" ", 1)
                if len(parts) != 2:
                    continue
                a, b = parts
            else:
                continue
            lines.append(f'{i} = ["{toml_escape(a)}", "{toml_escape(b)}"]')

    lines.append("")
    return "\n".join(lines)


def patch_model_file(model_path: str, new_vocab: str):
    """Replace ~~~vocab section in .model file with correct content."""
    with open(model_path, "rb") as f:
        data = f.read()

    # Find ~~~vocab and ~~~eval (or ~~~weights) markers
    vocab_marker = b"~~~vocab\n"
    eval_marker = b"~~~eval\n"
    weights_marker = b"~~~weights\n"

    vocab_start = data.find(vocab_marker)
    if vocab_start == -1:
        print(f"  SKIP — no ~~~vocab section")
        return False

    content_start = vocab_start + len(vocab_marker)

    # Find the next section after vocab
    next_section = len(data)
    for marker in [eval_marker, weights_marker]:
        pos = data.find(marker, content_start)
        if pos != -1 and pos < next_section:
            next_section = pos

    # Replace vocab content
    new_data = data[:content_start] + new_vocab.encode("utf-8") + data[next_section:]

    with open(model_path, "wb") as f:
        f.write(new_data)

    return True


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 fix-vocab.py <llm_dir> [model_name]")
        sys.exit(1)

    llm_dir = sys.argv[1]
    filter_name = sys.argv[2] if len(sys.argv) > 2 else None

    print(f"fix-vocab: regenerating ~~~vocab sections\n")

    for fname in sorted(os.listdir(llm_dir)):
        if not fname.endswith(".model"):
            continue
        name = fname[:-6]  # strip .model

        if filter_name and name != filter_name:
            continue

        model_path = os.path.join(llm_dir, fname)
        tok_path = os.path.join(llm_dir, f"{name}.tokenizer.json")

        if not os.path.exists(tok_path):
            print(f"  {name} — no tokenizer.json, skipping")
            continue

        print(f"  {name}", end="")

        new_vocab = generate_vocab_toml(tok_path)
        if not new_vocab:
            print(" — empty vocab generated, skipping")
            continue

        token_count = new_vocab.count("\n") - 2  # rough count
        if patch_model_file(model_path, new_vocab):
            # Count tokens and merges
            n_tok = sum(1 for l in new_vocab.split("\n") if l and not l.startswith("[") and not l.startswith("#") and "=" in l and not l.startswith("0 = ["))
            print(f" — patched ({n_tok} entries)")
        else:
            print()

    print("\ndone.")


if __name__ == "__main__":
    main()
