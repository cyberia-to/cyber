---
tags: cyberia, operation, ai
crystal-type: entity
crystal-domain: cyberia
---

a first-principles architecture for distributed local intelligence on constrained hardware.

hardware target: Apple M1 Pro, 16GB unified memory, 1TB SSD.

based on how nature solves the problem: the brain runs ~86 billion neurons in specialized regions at different timescales and energy costs. the always-on substrate is small, fast, and parallel. the on-demand layer is large, slow, and sequential. total energy managed by keeping the expensive parts mostly off.

## the fundamental constraint

```
available RAM ≈ 13GB (after OS + processes)

always-on models must fit simultaneously:
  Σ(tier_0_models) ≤ 2GB

on-demand models load/unload:
  max(single_model_footprint) ≤ 10GB

working memory (KV cache, context):
  reserved ≥ 2GB
```

## four-tier cognitive stack

```
┌─────────────────────────────────────────────────┐
│  TIER 4: EXTERNAL ORACLE                         │
│  Claude API / Perplexity                         │
│  <5% of queries. irreversible decisions only.    │
└─────────────────────┬───────────────────────────┘
                      │ escalation only
┌─────────────────────▼───────────────────────────┐
│  TIER 3: DEEP SYNTHESIS (4 models, 13-14B)      │
│  load: 8-12s. one at a time. footprint: 10GB    │
└─────────────────────┬───────────────────────────┘
                      │ when tier 2 insufficient
┌─────────────────────▼───────────────────────────┐
│  TIER 2: DOMAIN REASONERS (12 models, 7-8B)     │
│  load: 3-6s. one at a time. footprint: 5-6GB    │
└─────────────────────┬───────────────────────────┘
                      │ when tier 1 insufficient
┌─────────────────────▼───────────────────────────┐
│  TIER 1: FAST SPECIALISTS (16 models, 1-3B)     │
│  load: 1-2s. footprint: 1-2GB                   │
└─────────────────────┬───────────────────────────┘
                      │ always available
┌─────────────────────▼───────────────────────────┐
│  TIER 0: COGNITIVE SUBSTRATE (8 models, <1B)     │
│  always loaded. total footprint: ~1.5GB          │
└─────────────────────────────────────────────────┘
```

## tier 0 — cognitive substrate (always parallel)

| # | role | params | function |
|---|------|--------|----------|
| 0.1 | input router | ~620M | classify every input before processing |
| 0.2 | embedding engine | ~239M | continuous vectorization for memory lookup |
| 0.3 | urgency scorer | ~184M | triage without wasting large model cycles |
| 0.4 | language detector | ~5M | natural + programming languages (EN/RU/ID/Balinese + 54 PLs) |
| 0.5 | intent extractor | ~494M | canonical form before downstream |
| 0.6 | anomaly detector | ~151M | sensor/log streams — always watching |
| 0.7 | context splitter | ~360M | manage context window before tier 1-3 |
| 0.8 | injection detector | ~163M | external input only — owner input bypasses this slot |

collective latency target: <100ms per input.

## tier 1 — fast specialists (16 models, sequential)

| # | role | params | function |
|---|------|--------|----------|
| 1.1 | code reviewer | 1.5B | static analysis, bug detection, style |
| 1.2 | SQL generator | 1.5B | natural language → SQL |
| 1.3 | translator | 3B | EN↔RU↔ID↔ZH domain-specific |
| 1.4 | summarizer | 3B | long-form → structured summary |
| 1.5 | entity extractor | 1.5B | people, places, quantities, dates |
| 1.6 | inventory parser | 1B | "+5 cement sector-B" → structured delta |
| 1.7 | sensor interpreter | 1B | raw telemetry → event + recommendation |
| 1.8 | financial parser | 1.5B | transactions → structured records |
| 1.9 | search query gen | 1B | intent → optimized queries |
| 1.10 | task decomposer | 3B | goal → ordered subtasks |
| 1.11 | report formatter | 3B | structured data → formatted output |
| 1.12 | alert composer | 1B | event → alert with correct severity |
| 1.13 | command parser | 1.5B | natural language → action JSON |
| 1.14 | memory retriever | 1.5B | query → relevant memory chunks |
| 1.15 | diff generator | 1.5B | before/after → changelog |
| 1.16 | schedule optimizer | 3B | tasks + constraints → schedule |

## tier 2 — domain reasoners (12 models, sequential)

| # | domain | params |
|---|--------|--------|
| 2.1 | general reasoner | 8B |
| 2.2 | code generator | 7B |
| 2.3 | research analyst | 8B |
| 2.4 | project planner | 7B |
| 2.5 | social dynamics | 7B |
| 2.6 | financial analyst | 7B |
| 2.7 | infrastructure ops | 8B |
| 2.8 | biology / permaculture | 7B |
| 2.9 | legal / compliance | 7B |
| 2.10 | creative / comms | 8B |
| 2.11 | mathematics | 7B |
| 2.12 | vision analyst | 8B |

## tier 3 — deep synthesis (4 models, sequential)

| # | role | params | when activated |
|---|------|--------|----------------|
| 3.1 | master coder | 14B | large codebase changes, architecture |
| 3.2 | strategic reasoner | 14B | cross-domain decisions, system design |
| 3.3 | deep generalist | 13B | novel problems beyond tier 2 |
| 3.4 | synthesis writer | 14B | long-form, whitepapers, complex comms |

## tier 4 — external oracle

| # | service | when invoked |
|---|---------|-------------|
| 4.1 | Claude API | irreversible decisions, novel strategy |
| 4.2 | Perplexity | real-time info, time-sensitive verification |

never invoked automatically. requires explicit routing decision with logged justification.

## concrete model selection

### tier 0

all models uncensored by design: generative models abliterated (refusal vectors removed from weights), encoder/classifier models produce scores/vectors with no refusal mechanism.

runtime stack: ONNX Runtime (7 slots) + native Rust (1 slot). zero Python, zero PyTorch, zero TensorFlow.

convert commands for models without ONNX in repo:
```
optimum-cli export onnx --model huihui-ai/Qwen3-0.6B-abliterated ./onnx/router/
optimum-cli export onnx --model huihui-ai/Qwen2.5-0.5B-Instruct-abliterated-v3 ./onnx/intent/
optimum-cli export onnx --model ibm-granite/granite-guardian-hap-125m ./onnx/injection-125m/
optimum-cli export onnx --model ibm-granite/granite-guardian-hap-38m ./onnx/injection-38m/
```

| slot | model | runtime | context | RAM | latency | notes |
|------|-------|---------|---------|-----|---------|-------|
| 0.1 router | [qwen3-0.6b-abliterated](https://huggingface.co/huihui-ai/Qwen3-0.6B-abliterated) | ONNX (convert) | 40K | ~350MB | ~15ms | LLM router — the reason modern agents work. abliterated, dual-mode (thinking/fast), constrained JSON output |
| 0.2 embedding | [jina-embeddings-v5-text-nano](https://huggingface.co/jinaai/jina-embeddings-v5-text-nano-retrieval) | ONNX (in repo) | 32K | ~180MB | ~12ms | 239M, 768-dim, matryoshka, task LoRA adapters, 119+ languages |
| 0.3 urgency | [deberta-v3-base-zeroshot-v2.0](https://huggingface.co/MoritzLaurer/deberta-v3-base-zeroshot-v2.0) | ONNX (in repo) | 512 | ~140MB | <5ms | zero-shot NLI classifier, any labels without fine-tuning |
| 0.4 language | [glotlid-v3](https://huggingface.co/cis-lmu/glotlid) + [hyperpolyglot](https://github.com/monkslc/hyperpolyglot) | native Rust | n/a | ~5MB | <1ms | fasttext-rs loads .bin directly. 2102 natural langs (incl. Balinese) + 100+ programming langs (Rust port of GitHub Linguist) |
| 0.5 intent | [qwen2.5-0.5b-abliterated-v3](https://huggingface.co/huihui-ai/Qwen2.5-0.5B-Instruct-abliterated-v3) | ONNX (convert) | 32K | ~350MB | ~15ms | 0% refusal rate on 320 harmful-instruction tests, constrained JSON |
| 0.6 anomaly | [tranad](https://github.com/imperial-qore/TranAD) + [modernbert-base](https://huggingface.co/answerdotai/ModernBERT-base) | ONNX (convert + in repo) | 8K | ~120MB | ~10ms | tranad: torch.onnx.export one-liner. modernbert: 8 ONNX variants in repo |
| 0.7 splitter | [smollm2-360m-instruct](https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct) | ONNX (in repo) | 8K | ~200MB | ~12ms | 4T tokens training, generative splitting with priority labels |
| 0.8 injection detector | [granite-guardian-hap-125m](https://huggingface.co/ibm-granite/granite-guardian-hap-125m) + [38m](https://huggingface.co/ibm-granite/granite-guardian-hap-38m) | ONNX (convert) | 512 | ~130MB | <3ms | external input only. owner input bypasses completely. binary classifier, owner sets threshold |
| | | | total: | ~1.38GB | <40ms | all 8 run in parallel, critical path ~15ms GPU |

### tier 1

| slot | model | source |
|------|-------|--------|
| 1.1 code review | qwen2.5-coder-1.5b | Alibaba |
| 1.2 SQL | sqlcoder-1.5b | Defog |
| 1.3 translator | madlad400-3b | Google |
| 1.4 summarizer | qwen2.5-3b | Alibaba |
| 1.5 entity | gliner-large | NuMind |
| 1.6 inventory | qwen2.5-0.5b (fine-tuned) | Alibaba |
| 1.7 sensor | smollm2-1.7b | HuggingFace |
| 1.8 financial | qwen2.5-1.5b | Alibaba |
| 1.9 search | qwen2.5-0.5b | Alibaba |
| 1.10 task decomp | llama-3.2-3b | Meta |
| 1.11 report | qwen2.5-3b | Alibaba |
| 1.12 alert | smollm2-360m | HuggingFace |
| 1.13 cmd parser | qwen2.5-1.5b | Alibaba |
| 1.14 mem retrieval | nomic-embed + BM25 | Nomic |
| 1.15 diff gen | qwen2.5-1.5b | Alibaba |
| 1.16 schedule | llama-3.2-3b | Meta |

### tier 2

| slot | model | source |
|------|-------|--------|
| 2.1 general | deepseek-r1:8b | DeepSeek |
| 2.2 code | qwen2.5-coder:7b | Alibaba |
| 2.3 research | mistral-7b-v0.3 | Mistral |
| 2.4 planning | llama-3.1-8b | Meta |
| 2.5 social | llama-3.1-8b | Meta |
| 2.6 finance | qwen2.5-7b | Alibaba |
| 2.7 infra | qwen2.5-coder:7b | Alibaba |
| 2.8 biology | llama-3.1-8b | Meta |
| 2.9 legal | qwen2.5-7b | Alibaba |
| 2.10 creative | mistral-7b-v0.3 | Mistral |
| 2.11 math | qwen2.5-math-7b | Alibaba |
| 2.12 vision | llava-v1.6-mistral-7b | LLaVA |

### tier 3

| slot | model | source |
|------|-------|--------|
| 3.1 master coder | qwen2.5-coder:14b | Alibaba |
| 3.2 strategic | deepseek-r1:14b | DeepSeek |
| 3.3 deep general | qwen2.5:14b | Alibaba |
| 3.4 synthesis | mistral-nemo:12b | Mistral |

### tier 4

| slot | service | model |
|------|---------|-------|
| 4.1 oracle | Anthropic API | claude-sonnet-4-5 |
| 4.2 search | Perplexity API | sonar-pro |

## memory architecture

```
working memory    — KV cache, ephemeral, max 32K tokens
episodic memory   — vector store (ChromaDB/Qdrant), persistent, grows
semantic memory   — knowledge graph (cybergraph), persistent, structured
procedural memory — tool definitions (MCP servers), static
```

## RAM budget

```
tier 0 (always loaded):  ~1.5GB
tier 3 model (worst):    ~7.5GB (Q3_K_M)
KV cache + context:      ~2.5GB
OS + processes:           ~3.0GB
────────────────────────────────
total peak:              ~14.5GB  ✅ fits M1 Pro 16GB
```

## disk

```
tier 0:   ~2GB
tier 1:  ~28GB
tier 2:  ~58GB
tier 3:  ~35GB
─────────────
total:  ~124GB (1TB SSD: comfortable)
```

## escalation logic

```
input arrives
    │
    ▼
tier 0 processes (always, ~50ms)
    │
    ├── substrate answers directly? → done
    │
    ▼
tier 1 selected (structured task?)
    │
    ├── sufficient? → done
    │
    ▼
tier 2 selected (domain reasoning needed?)
    │
    ├── sufficient? → done
    │
    ▼
tier 3 activated (deep synthesis required?)
    │
    ├── sufficient? → done
    │
    ▼
tier 4 invoked (irreversible / strategic / novel?)
    └── answer + log decision + update memory
```

most queries never leave tier 1. tier 3 activates ~10-15%. tier 4 <5%.

## emergent properties

42 narrow models > 1 large model because:
- precision: specialist interprets anomalies better than generalist
- speed: 500M router is 50x faster than 14B for every request
- reliability: failures isolated. one model failing does not collapse the system
- evolvability: individual models swapped without rebuilding

intelligence accumulates in the memory layer, not the weights. routing logic IS the self of the system — mirrors the binding problem in neuroscience.

> "The whole is not the sum of the parts. It is the pattern of their interaction." — Gregory Bateson

[[cyber valley]] Leadership OS meets Local Mind v0.1
