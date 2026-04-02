#!/usr/bin/env nu
# migrate-model-phase2.nu — program.rs + vocab.toml for .model spec
#
# usage: nu analizer/migrate-model-phase2.nu ~/llm           # all models
#        nu analizer/migrate-model-phase2.nu ~/llm qwen3-0.6b-abl  # one model

# architecture family mapping: model_name → program template
def arch_map [] {
    {
        "qwen3-0.6b-abl":         "causal_lm"
        "qwen2.5-0.5b-abl":      "causal_lm"
        "qwen2.5-coder-1.5b-abl": "causal_lm"
        "qwen2.5-coder-14b-abl": "causal_lm"
        "qwen3.5-4b-abl":        "causal_lm"
        "qwen3.5-9b-abl":        "causal_lm"
        "deepseek-r1-8b-abl":    "causal_lm"
        "smollm2-360m":          "causal_lm"
        "bitnet-2b":             "causal_lm"
        "nuextract-1.5":         "causal_lm"
        "mimo-7b-rl":            "causal_lm"
        "deberta-zeroshot":      "encoder"
        "modernbert":            "encoder"
        "granite-hap-125m":      "encoder"
        "granite-hap-38m":       "encoder"
        "jina-v5-nano":          "encoder"
        "whisper-small":         "whisper"
        "yolo11n":               "yolo"
        "beats":                 "beats"
        "moondream2":            "vlm"
        "qwen2.5-vl-7b-abl":    "vlm"
        "piper-tts":             "tts"
        "xtts-v2":               "tts"
        "wan22-video":           "video_gen"
        "glotlid":               "fasttext"
    }
}

# copy program.rs from template
def copy_program [dir: string, model_name: string, programs_dir: string] {
    let mapping = arch_map
    let arch = $mapping | get $model_name
    let src = ($programs_dir | path join $"($arch).rs")
    let dst = ($dir | path join "program.rs")

    if not ($src | path exists) {
        print $"  WARN: no program template ($arch).rs"
        return
    }

    cp $src $dst
    print $"  program.rs — ($arch)"
}

# convert tokenizer.json → vocab.toml
def convert_vocab [dir: string] {
    let tok_path = ($dir | path join "tokenizer.json")
    if not ($tok_path | path exists) {
        print "  vocab.toml — no tokenizer.json, keeping existing"
        return
    }

    let tok = open $tok_path
    let model_type = $tok | get model.type

    # handle BPE (record vocab) vs Unigram/SentencePiece (list vocab)
    let vocab_raw = $tok | get model.vocab

    let token_lines = if $model_type == "BPE" {
        # BPE: vocab is {token: id}
        $vocab_raw
            | transpose key value
            | sort-by value
            | each {|r| $"($r.value) = \"($r.key | str replace --all '\\' '\\\\' | str replace --all '"' '\\"')\"" }
            | str join "\n"
    } else {
        # Unigram/SentencePiece: vocab is [[token, score], ...]
        $vocab_raw
            | enumerate
            | each {|r| $"($r.index) = \"($r.item.0 | str replace --all '\\' '\\\\' | str replace --all '"' '\\"')\"" }
            | str join "\n"
    }

    let token_count = if $model_type == "BPE" {
        $vocab_raw | transpose | length
    } else {
        $vocab_raw | length
    }

    # merges (BPE only, may not exist for Unigram)
    let has_merges = ($tok | get model | columns | any {|c| $c == "merges" })
    let merge_lines = if $has_merges {
        let merges = $tok | get model.merges
        if ($merges | length) > 0 {
            $merges
                | enumerate
                | each {|r|
                    let parts = $r.item | split row " "
                    let a = $parts | first | str replace --all '"' '\\"'
                    let b = $parts | skip 1 | str join " " | str replace --all '"' '\\"'
                    $"($r.index) = [\"($a)\", \"($b)\"]"
                }
                | str join "\n"
        } else { "" }
    } else { "" }

    let merge_count = if ($merge_lines | is-empty) { 0 } else { $merge_lines | lines | length }

    let output = if ($merge_lines | is-empty) {
        $"[tokens]\n($token_lines)\n"
    } else {
        $"[tokens]\n($token_lines)\n\n[merges]\n($merge_lines)\n"
    }

    $output | save -f ($dir | path join "vocab.toml")
    print $"  vocab.toml — ($token_count) tokens, ($merge_count) merges"
}

def migrate_one [llm_path: string, model_name: string, programs_dir: string] {
    let dir = ($llm_path | path join $model_name)
    if not ($dir | path exists) {
        print $"  SKIP ($model_name) — missing"
        return
    }

    # program.rs
    copy_program $dir $model_name $programs_dir

    # vocab.toml
    convert_vocab $dir

    print $"  OK ($model_name)"
}

def main [llm_path: string, model_name?: string] {
    let programs_dir = ($env.FILE_PWD | path join "programs")

    print $"phase 2: program.rs + vocab.toml\n"

    let models = arch_map | columns
    if ($model_name != null) {
        print $"── ($model_name) ──"
        migrate_one $llm_path $model_name $programs_dir
    } else {
        for m in $models {
            print $"── ($m) ──"
            migrate_one $llm_path $m $programs_dir
        }
    }

    print "\ndone. remaining:"
    print "  - tensors.toml (extract from .cyb binary — needs Rust)"
    print "  - pack into .model containers"
    print "  - flatten ~/llm/ to just .model files"
}
