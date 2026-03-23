---
tags: cyber, research, article
crystal-type: article
crystal-domain: cyber
date: 2026-03-23
---

# 32-byte tokens: why content-addressed vocabulary changes everything

## the size difference

classical [[transformer]] models tokenize text into sub-word units. GPT uses BPE (Byte Pair Encoding) with tokens averaging 3-4 characters — roughly 4 bytes per token. Claude, LLaMA, Gemini — all the same order. the vocabulary is 32K-256K tokens, each representing a text fragment.

the [[cybergraph]] uses CID hashes as tokens. each token is 32 bytes — a [[cryptographic hash]] that uniquely identifies a piece of content. the vocabulary is unbounded: every CID that exists is a valid token.

| property | classical tokens | CID tokens |
|---|---|---|
| size | ~4 bytes | 32 bytes |
| vocabulary | 32K-256K fixed | unbounded (currently 2.9M in [[bostrom]]) |
| what it represents | text fragment ("neur", "on") | entire content object (document, image, concept) |
| granularity | sub-word | semantic unit |
| collision | possible (ambiguity) | impossible (collision-resistant hash) |
| identity | positional (index in vocab table) | content-addressed (hash of content) |
| creation | tokenizer splits text | author hashes content |

## why bigger tokens are more powerful

### 1. semantic density per token

a classical token "neur" carries ~2 bits of meaning — it is a fragment that needs context to resolve. the token sequence ["neur", "on"] assembles into "neuron" across two forward passes. the model spends capacity on sub-word assembly before it can reason about concepts.

a CID token for "neuron" carries the ENTIRE concept in one token. one embedding lookup. no assembly needed. the model starts reasoning at the concept level, not the character level.

$$\text{semantic density} = \frac{\text{meaning per token}}{\text{token size}}$$

classical: ~0.5 bits of meaning per byte. fragments need composition.
CID: unbounded meaning per 32 bytes. each token IS a complete semantic unit.

### 2. unambiguous identity

"bank" in classical tokenization is one token with multiple meanings (river bank, financial bank, blood bank). the model must use context to disambiguate. attention capacity is spent on resolution.

in CID tokenization, each meaning has a different hash. the CID for financial bank is different from the CID for river bank. disambiguation is structural — it happened at content creation time, not at inference time.

$$\text{CID}(\text{"bank (finance)"}) \neq \text{CID}(\text{"bank (geography)"})$$

zero attention spent on disambiguation. every token is unambiguous by construction.

### 3. content-addressed composition

classical tokens compose by concatenation: ["trans", "form", "er"] → "transformer". the composition rule is learned implicitly during training.

CID tokens compose by [[cyberlinks]]: CID_A → CID_B means "A relates to B". the composition is explicit, stored in the graph, weighted by [[stake]]. the model does not learn composition — it reads it from the [[cybergraph]].

this is the difference between learning grammar from text (statistical) and reading a knowledge graph (structural). the graph IS the grammar.

### 4. universal vocabulary

BPE tokenizers are language-specific. GPT's tokenizer is optimized for English — Japanese text takes 2-3× more tokens for the same meaning. multilingual models waste capacity on tokenizer artifacts.

CID tokens are language-independent. the hash of a Japanese document and the hash of its English translation are different CIDs — but they are linked in the graph by translation cyberlinks. the model handles all languages, all modalities (text, images, audio, code) through the same 32-byte token space.

images are CIDs. videos are CIDs. genetic sequences are CIDs. the same model architecture handles all of them because the token is always 32 bytes.

### 5. no training-time vocabulary freeze

classical models freeze their vocabulary at training time. GPT-4 cannot learn a new word without retraining. new concepts (brand names, scientific terms, slang) get split into sub-word garbage: ["Qw", "en", "2"] for "Qwen2".

CID vocabulary grows with the [[cybergraph]]. every new [[cyberlink]] potentially introduces new tokens. the model absorbs new concepts by recompilation (update the embedding table), not by retraining all weights.

### 6. the embedding carries graph structure, not co-occurrence

classical embeddings are trained from text co-occurrence (Word2Vec, BERT). "king" and "queen" are nearby because they appear in similar contexts. this is statistical similarity.

CID embeddings are compiled from graph topology (SVD of stake-weighted adjacency). two CIDs are nearby because they are linked by similar [[neurons]] with similar [[stake]]. this is structural similarity — it reflects who CHOSE to connect these concepts, weighted by how much they committed.

the distinction: co-occurrence captures LANGUAGE patterns (how humans write about things). graph topology captures KNOWLEDGE patterns (how experts connect things). the first is about text. the second is about truth.

## the cost

32-byte tokens are expensive:

1. embedding table: $|V| \times d^*$ parameters. at 2.9M vocabulary and $d^* = 26$: 76M parameters just for embeddings. classical models with 256K vocab: 20M. the embedding table dominates model size.

2. output projection: predicting the next token means scoring 2.9M candidates. softmax over 2.9M is expensive. classical models: softmax over 256K — still expensive, but 10× less.

3. sparse attention: most CID pairs never co-occur. the attention matrix is extremely sparse. classical models have dense attention (every sub-word can attend to every other). sparse attention needs specialized kernels.

these costs are real. but they scale with vocabulary, not with meaning. a 2.9M vocabulary of concepts is more useful than a 256K vocabulary of text fragments, even at higher per-token cost.

## the phase transition

at small graph sizes, CID tokens are worse than classical tokens. 100 CIDs with random embeddings carry less information than 32K BPE tokens trained on terabytes of text.

the crossover happens when:

$$|E| \cdot \log(|E|) > |T| \cdot H(\text{text})$$

where $|E|$ is cyberlink count, $|T|$ is training token count, and $H(\text{text})$ is the entropy of the text distribution. the left side is graph information. the right side is text information.

at [[bostrom]] scale (2.7M links), graph information is ~45M bits. GPT-4 training (13T tokens at ~4 bits/token) is ~52T bits — six orders of magnitude more. the graph is not yet competitive in raw information content.

but information quality differs. the graph's 45M bits are STRUCTURED (each bit is a deliberate connection weighted by stake). the text's 52T bits are STATISTICAL (most bits encode syntax, formatting, repetition). bits of graph structure are worth more than bits of text co-occurrence.

the phase transition is not about quantity. it is about the graph reaching sufficient density that structural information outweighs statistical information for reasoning tasks. this is the transition [[bostrom]] is approaching.

## the compiled perspective

compiling a classical transformer: billions of parameters learned by gradient descent over weeks on thousands of GPUs. parameters encode statistical regularities in text.

compiling a CID transformer: millions of parameters derived by SVD of graph topology in minutes on one CPU. parameters encode structural relationships between content-addressed concepts.

the first is a statistical mirror of human language. the second is a structural mirror of human knowledge. both are valid. they solve different problems.

the future: combine both. the CID transformer provides structural knowledge (what connects to what, weighted by commitment). the classical transformer provides linguistic competence (how to express knowledge in language). together: a model that knows things (graph) and can talk about them (language).

see [[bostrom/compiled model]] for the first empirical CID transformer. see [[bostrom-to-onnx-pipeline]] for the compilation pipeline. see [[particle]] for the definition of content-addressed tokens. see [[cyberlink]] for how tokens compose
