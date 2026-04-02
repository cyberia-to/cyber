#!/usr/bin/env nu
# migrate-model.nu — migrate ~/llm models to .model spec config layer
#
# usage: nu analizer/migrate-model.nu ~/llm           # migrate all
#        nu analizer/migrate-model.nu ~/llm qwen3-0.6b-abl  # migrate one

# manifest metadata (from llm/src/manifest.rs)
def manifest [] {
    [
        { name: "qwen3-0.6b-abl",          hf_repo: "huihui-ai/Qwen3-0.6B-abliterated",                    tier: "tier0", role: "router",         license: "Apache-2.0", params: 600000000,   notes: "LLM router, dual-mode" }
        { name: "jina-v5-nano",             hf_repo: "jinaai/jina-embeddings-v5-text-nano-retrieval",        tier: "tier0", role: "embedding",       license: "Apache-2.0", params: 239000000,   notes: "768-dim matryoshka embeddings" }
        { name: "deberta-zeroshot",         hf_repo: "MoritzLaurer/deberta-v3-base-zeroshot-v2.0",           tier: "tier0", role: "urgency",         license: "MIT",        params: 184000000,   notes: "Zero-shot NLI classifier" }
        { name: "glotlid",                  hf_repo: "cis-lmu/glotlid",                                     tier: "tier0", role: "language",        license: "Apache-2.0", params: 0,            notes: "fasttext, 2102 natural languages" }
        { name: "qwen2.5-0.5b-abl",        hf_repo: "huihui-ai/Qwen2.5-0.5B-Instruct-abliterated-v3",      tier: "tier0", role: "intent",          license: "Apache-2.0", params: 500000000,   notes: "Intent extraction, constrained JSON" }
        { name: "modernbert",               hf_repo: "answerdotai/ModernBERT-base",                          tier: "tier0", role: "anomaly",         license: "Apache-2.0", params: 150000000,   notes: "Anomaly detection backbone" }
        { name: "smollm2-360m",             hf_repo: "HuggingFaceTB/SmolLM2-360M-Instruct",                  tier: "tier0", role: "splitter",        license: "Apache-2.0", params: 360000000,   notes: "Generative splitting with priority labels" }
        { name: "granite-hap-125m",         hf_repo: "ibm-granite/granite-guardian-hap-125m",                 tier: "tier0", role: "injection",       license: "Apache-2.0", params: 125000000,   notes: "Prompt injection binary classifier" }
        { name: "granite-hap-38m",          hf_repo: "ibm-granite/granite-guardian-hap-38m",                  tier: "tier0", role: "injection",       license: "Apache-2.0", params: 38000000,    notes: "Prompt injection binary classifier" }
        { name: "bitnet-2b",                hf_repo: "microsoft/bitnet-b1.58-2B-4T",                         tier: "tier1", role: "workhorse",       license: "MIT",        params: 2000000000,  notes: "Native 1.58-bit, 4T tokens" }
        { name: "qwen3.5-4b-abl",          hf_repo: "huihui-ai/Huihui-Qwen3.5-4B-abliterated",              tier: "tier1", role: "generalist",      license: "Apache-2.0", params: 4000000000,  notes: "4B generalist" }
        { name: "nuextract-1.5",            hf_repo: "numind/NuExtract-1.5",                                 tier: "tier1", role: "extraction",      license: "MIT",        params: 1500000000,  notes: "Entity extraction specialist" }
        { name: "qwen2.5-coder-1.5b-abl",  hf_repo: "huihui-ai/Qwen2.5-Coder-1.5B-Instruct-abliterated",   tier: "tier1", role: "coder-small",     license: "Apache-2.0", params: 1500000000,  notes: "Code specialist, small" }
        { name: "qwen3.5-9b-abl",          hf_repo: "huihui-ai/Huihui-Qwen3.5-9B-abliterated",              tier: "tier2", role: "generalist",      license: "Apache-2.0", params: 9000000000,  notes: "General reasoning" }
        { name: "qwen2.5-coder-14b-abl",   hf_repo: "huihui-ai/Qwen2.5-Coder-14B-Instruct-abliterated",    tier: "tier2", role: "coder",           license: "Apache-2.0", params: 14000000000, notes: "Code generation, SQL" }
        { name: "mimo-7b-rl",               hf_repo: "XiaomiMiMo/MiMo-7B-RL",                               tier: "tier2", role: "reasoning-math",  license: "Apache-2.0", params: 7000000000,  notes: "Math reasoning, AIME 55.4" }
        { name: "deepseek-r1-8b-abl",      hf_repo: "huihui-ai/DeepSeek-R1-0528-Qwen3-8B-abliterated",      tier: "tier2", role: "reasoning-cot",   license: "MIT",        params: 8000000000,  notes: "Chain-of-thought reasoning" }
        { name: "qwen2.5-vl-7b-abl",       hf_repo: "huihui-ai/Qwen2.5-VL-7B-Instruct-abliterated",        tier: "tier2", role: "vision",          license: "Apache-2.0", params: 7000000000,  notes: "Vision + video, OCR" }
        { name: "whisper-small",            hf_repo: "",                                                      tier: "media", role: "stt",             license: "MIT",        params: 244000000,   notes: "Speech-to-text, whisper.cpp" }
        { name: "piper-tts",               hf_repo: "",                                                      tier: "media", role: "tts",             license: "MIT",        params: 60000000,    notes: "VITS ONNX voices" }
        { name: "yolo11n",                  hf_repo: "",                                                      tier: "media", role: "detection",       license: "AGPL-3.0",   params: 3200000,     notes: "YOLOv11 nano" }
        { name: "beats",                    hf_repo: "camenduru/beats",                                       tier: "media", role: "audio-events",    license: "CC-BY-NC-4.0", params: 90000000, notes: "Audio event detection, 527 AudioSet classes" }
        { name: "moondream2",               hf_repo: "vikhyatk/moondream2",                                  tier: "media", role: "vision-light",    license: "Apache-2.0", params: 1800000000,  notes: "Lightweight vision Q&A" }
        { name: "xtts-v2",                  hf_repo: "coqui/XTTS-v2",                                        tier: "media", role: "voice-clone",     license: "CPML",       params: 467000000,   notes: "Voice cloning EN+RU" }
        { name: "wan22-video",              hf_repo: "QuantStack/Wan2.2-TI2V-5B-GGUF",                       tier: "media", role: "video-gen",       license: "Apache-2.0", params: 5000000000,  notes: "Text/image to video generation" }
    ]
}

# convert float to integer: rms_norm_eps → inverse
def float_to_inverse [val: float] {
    if $val == 0.0 { return 0 }
    (1.0 / $val) | math round | into int
}

# convert float to scaled integer (× 1000)
def float_to_scaled [val: float] {
    ($val * 1000.0) | math round | into int
}


# generate spec-compliant config.toml
def generate_config [dir: string, meta: record] {
    let config_path = ($dir | path join "config.toml")
    if not ($config_path | path exists) { return null }

    let old_config = open --raw $config_path
    let lines = $old_config | lines

    # parse existing config.toml (flat key = value)
    mut arch_fields = []
    mut model_type = ""
    mut top_fields = {}

    for line in $lines {
        let l = $line | str trim
        if ($l | is-empty) or ($l | str starts-with "#") or ($l | str starts-with "[") { continue }
        if not ($l | str contains "=") { continue }

        let parts = $l | split row "=" | each { str trim }
        let key = $parts | first
        let val = $parts | skip 1 | str join "=" | str trim

        if $key == "model_type" {
            $model_type = ($val | str trim --char '"')
        } else if $key == "architecture" {
            # skip HF class name
        } else if $key == "tie_word_embeddings" {
            # skip, not needed in spec
        } else if $key == "rms_norm_eps" {
            let fval = ($val | into float)
            let ival = if $fval > 0.0 and $fval < 1.0 {
                1.0 / $fval | math round | into int
            } else if $fval >= 1.0 {
                $fval | math round | into int
            } else { 0 }
            $arch_fields = ($arch_fields | append $"rms_norm_eps = ($ival)")
        } else if $key in ["hidden_size", "num_attention_heads", "num_key_value_heads",
                           "num_hidden_layers", "intermediate_size", "vocab_size",
                           "max_position_embeddings", "rope_theta", "head_dim",
                           "num_classes", "input_size", "num_labels", "input_sample_rate",
                           "num_mels", "context_length"] {
            $arch_fields = ($arch_fields | append $"($key) = ($val)")
        } else if $key == "variant" {
            $arch_fields = ($arch_fields | append $"($key) = ($val)")
        }
    }

    # read sampling.toml if exists
    let sampling_path = ($dir | path join "sampling.toml")
    mut sampling_fields = []
    if ($sampling_path | path exists) {
        let sdata = open --raw $sampling_path | lines
        for line in $sdata {
            let l = $line | str trim
            if ($l | is-empty) or ($l | str starts-with "#") { continue }
            if not ($l | str contains "=") { continue }
            let parts = $l | split row "=" | each { str trim }
            let key = $parts | first
            let val = $parts | skip 1 | str join "=" | str trim

            if $key == "temperature" {
                let scaled = float_to_scaled ($val | into float)
                $sampling_fields = ($sampling_fields | append $"temperature = ($scaled)")
            } else if $key == "top_p" {
                let scaled = float_to_scaled ($val | into float)
                $sampling_fields = ($sampling_fields | append $"top_p = ($scaled)")
            } else if $key == "top_k" {
                $sampling_fields = ($sampling_fields | append $"top_k = ($val)")
            } else if $key == "repetition_penalty" {
                let scaled = float_to_scaled ($val | into float)
                $sampling_fields = ($sampling_fields | append $"repetition_penalty = ($scaled)")
            } else if $key == "eos_token_ids" {
                # skip, goes to tokenizer
            }
        }
        if ($sampling_fields | length) > 0 {
            $sampling_fields = ($sampling_fields | append "scale = 1000")
        }
    }

    # read vocab.toml if exists
    let vocab_path = ($dir | path join "vocab.toml")
    mut tokenizer_fields = []
    if ($vocab_path | path exists) {
        let vdata = open --raw $vocab_path | lines
        for line in $vdata {
            let l = $line | str trim
            if ($l | is-empty) or ($l | str starts-with "#") { continue }
            if not ($l | str contains "=") { continue }
            let parts = $l | split row "=" | each { str trim }
            let key = $parts | first
            let val = $parts | skip 1 | str join "=" | str trim
            if $key == "type" {
                $tokenizer_fields = ($tokenizer_fields | append $"type = ($val)")
            }
        }
    }

    # read chat.toml for eos/pad tokens
    let chat_path = ($dir | path join "chat.toml")
    if ($chat_path | path exists) {
        let cdata = open --raw $chat_path | lines
        for line in $cdata {
            let l = $line | str trim
            if ($l | is-empty) or ($l | str starts-with "#") { continue }
            if not ($l | str contains "=") { continue }
            let parts = $l | split row "=" | each { str trim }
            let key = $parts | first
            let val = $parts | skip 1 | str join "=" | str trim
            if $key == "eos_token" or $key == "pad_token" {
                $tokenizer_fields = ($tokenizer_fields | append $"($key) = ($val)")
            }
        }
    }

    # read eos_token_ids from sampling.toml
    if ($sampling_path | path exists) {
        let sdata = open --raw $sampling_path | lines
        for line in $sdata {
            let l = $line | str trim
            if ($l | str starts-with "eos_token_ids") {
                let parts = $l | split row "=" | each { str trim }
                let val = $parts | skip 1 | str join "=" | str trim
                $tokenizer_fields = ($tokenizer_fields | append $"eos_token_ids = ($val)")
            }
        }
    }

    let params = $meta.params

    # determine abliteration
    let method = if ($meta.name | str contains "-abl") { "abliteration" } else { "" }

    # build output
    mut output = $"model_type = \"($model_type)\"\n"
    if $params > 0 {
        $output = $output + $"parameters = ($params)\n"
    }
    $output = $output + $"license = \"($meta.license)\"\n"

    # [architecture]
    if ($arch_fields | length) > 0 {
        $output = $output + "\n[architecture]\n"
        for f in $arch_fields {
            $output = $output + $f + "\n"
        }
    }

    # [tokenizer]
    if ($tokenizer_fields | length) > 0 {
        $output = $output + "\n[tokenizer]\n"
        for f in $tokenizer_fields {
            $output = $output + $f + "\n"
        }
    }

    # [sampling]
    if ($sampling_fields | length) > 0 {
        $output = $output + "\n[sampling]\n"
        for f in $sampling_fields {
            $output = $output + $f + "\n"
        }
    }

    # [lineage]
    $output = $output + "\n[lineage]\n"
    if ($meta.hf_repo | is-not-empty) {
        $output = $output + $"source = \"($meta.hf_repo)\"\n"
    }
    if ($method | is-not-empty) {
        $output = $output + $"method = \"($method)\"\n"
    }

    $output
}

# generate card.md
def generate_card [meta: record, dir: string] {
    let cyb_path = ($dir | path join $"($meta.name).cyb")
    let size_str = if ($cyb_path | path exists) {
        let bytes = ls $cyb_path | first | get size | into int
        if $bytes >= 1_000_000_000 {
            $"(($bytes | into float) / 1_000_000_000.0 | math round --precision 1)GB"
        } else {
            $"(($bytes | into float) / 1_000_000.0 | math round | into int)MB"
        }
    } else { "unknown" }

    let method_line = if ($meta.name | str contains "-abl") {
        "\nabliterated: refusal vectors removed.\n"
    } else { "" }

    $"# ($meta.name)\n\n($meta.notes). ($size_str) on disk.\nsoma ($meta.tier), role: ($meta.role).\n($method_line)\nsource: ($meta.hf_repo)\nlicense: ($meta.license)\n"
}

# migrate a single model
def migrate_one [llm_path: string, model_name: string] {
    let dir = ($llm_path | path join $model_name)
    if not ($dir | path exists) {
        print $"  SKIP ($model_name) — directory missing"
        return
    }

    let meta = manifest | where name == $model_name | first
    if ($meta | is-empty) {
        print $"  SKIP ($model_name) — not in manifest"
        return
    }

    # generate config
    let new_config = generate_config $dir $meta
    if ($new_config != null) {
        $new_config | save -f ($dir | path join "config.toml")
        print $"  config.toml — merged + integers"
    }

    # generate card.md
    let card = generate_card $meta $dir
    $card | save -f ($dir | path join "card.md")
    print $"  card.md — generated"

    # generate eval.toml (empty)
    "{}\n" | save -f ($dir | path join "eval.toml")
    print $"  eval.toml — stub"

    # cleanup old scattered files
    for old_file in ["chat.toml", "sampling.toml"] {
        let old_path = ($dir | path join $old_file)
        if ($old_path | path exists) {
            rm $old_path
            print $"  ($old_file) — removed, merged into config.toml"
        }
    }

    print $"  OK ($model_name)"
}

def main [llm_path: string, model_name?: string] {
    print $"migrating models in ($llm_path) to .model spec...\n"

    if ($model_name != null) {
        print $"── ($model_name) ──"
        migrate_one $llm_path $model_name
    } else {
        let models = manifest | get name
        for m in $models {
            print $"── ($m) ──"
            migrate_one $llm_path $m
        }
    }

    print "\ndone. remaining for phase 2 (Rust tooling):"
    print "  - tensors.toml (extract from .cyb weights)"
    print "  - vocab.toml (convert tokenizer.json → full tokens + merges)"
    print "  - program.tri (generate per architecture)"
    print "  - repack into .model containers"
}
