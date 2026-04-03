#!/usr/bin/env nu
# fetch-tokenizers.nu — download tokenizer.json for each .model file
#
# usage: nu analizer/fetch-tokenizers.nu ~/llm

# model name → HuggingFace repo with tokenizer.json
# abliterated models share tokenizer with base model
def source_map [] {
    {
        "qwen3-0.6b-abl":         "Qwen/Qwen3-0.6B"
        "qwen2.5-0.5b-abl":      "Qwen/Qwen2.5-0.5B"
        "qwen2.5-coder-1.5b-abl": "Qwen/Qwen2.5-Coder-1.5B"
        "qwen2.5-coder-14b-abl": "Qwen/Qwen2.5-Coder-14B"
        "qwen3.5-4b-abl":        "Qwen/Qwen3-4B"
        "qwen3.5-9b-abl":        "Qwen/Qwen3-8B"
        "deepseek-r1-8b-abl":    "deepseek-ai/DeepSeek-R1-0528-Qwen3-8B"
        "smollm2-360m":          "HuggingFaceTB/SmolLM2-360M-Instruct"
        "bitnet-2b":             "microsoft/bitnet-b1.58-2B-4T"
        "nuextract-1.5":         "numind/NuExtract-1.5"
        "mimo-7b-rl":            "XiaomiMiMo/MiMo-7B-RL"
        "deberta-zeroshot":      "MoritzLaurer/deberta-v3-base-zeroshot-v2.0"
        "modernbert":            "answerdotai/ModernBERT-base"
        "granite-hap-125m":      "ibm-granite/granite-guardian-hap-125m"
        "granite-hap-38m":       "ibm-granite/granite-guardian-hap-38m"
        "jina-v5-nano":          "jinaai/jina-embeddings-v5-text-nano-retrieval"
        "whisper-small":         "openai/whisper-small"
        "moondream2":            "vikhyatk/moondream2"
        "qwen2.5-vl-7b-abl":    "Qwen/Qwen2.5-VL-7B-Instruct"
    }
}

def main [llm_path: string] {
    let models = source_map
    print $"Fetching tokenizer.json files → ($llm_path)\n"

    for model_name in ($models | columns) {
        let repo = $models | get $model_name
        let dst = ($llm_path | path join $"($model_name).tokenizer.json")

        if ($dst | path exists) {
            print $"  ($model_name) — already exists"
            continue
        }

        print $"  ($model_name) ← ($repo)"
        try {
            let result = (python3 -c $"
from huggingface_hub import hf_hub_download
path = hf_hub_download\('($repo)', 'tokenizer.json'\)
print\(path\)
" | str trim)
            cp $result $dst
            let size = (ls $dst | get size | first)
            print $"    ✓ ($size)"
        } catch {
            print $"    ✗ download failed"
        }
    }

    print "\ndone."
}
