---
alias: fine-tuning, finetuning, model fine-tuning, transfer learning
tags: cybics, article, research
crystal-type: pattern
crystal-domain: computer science
---

adapting a pre-trained [[neural networks|neural network]] to a specific task by continuing [[learning]] on a narrow dataset. the core technique behind specialization of [[llms|language models]] and other foundation models.

---

## the mechanism

a foundation model is trained on massive corpora — terabytes of text, weeks on GPU clusters, millions of dollars. the result is a weight matrix: billions of parameters encoding statistical patterns of language. this is [[pre-training]].

fine-tuning takes these weights as initialization and runs additional training on a small, curated dataset (hundreds to thousands of examples). technically:

1. load pre-trained weights into the [[transformer]] architecture
2. forward pass: input flows through layers, producing predictions
3. compute loss — the gap between model output and the desired output
4. [[backpropagation]]: compute gradients via chain rule through every layer
5. update weights via gradient descent — but with a much smaller learning rate than pre-training

the low learning rate is critical: large updates would overwrite the general knowledge encoded during pre-training. fine-tuning nudges the weight landscape, it does not rebuild it.

---

## what changes in the weights

each parameter is a floating point number in a matrix. fine-tuning shifts these numbers toward a region that minimizes error on the new examples. the model does not memorize data literally — it adjusts activation patterns across neurons so that the probability distribution over outputs aligns with the target dataset.

the loss surface of a pre-trained model is already in a good basin. fine-tuning moves within that basin (or to a nearby one) rather than traversing the full landscape from random initialization.

---

## methods

### full fine-tuning

all parameters are updated. requires GPU memory for the full model plus gradients plus optimizer state — roughly 3-4x the model size in VRAM. a 70B parameter model needs multiple GPUs with hundreds of gigabytes.

### LoRA (Low-Rank Adaptation)

freeze the original weights. inject small trainable matrices (adapters) beside each layer. these adapters have low rank (8-64), meaning they capture the most significant directions of change with minimal parameters — typically 0.1-1% of the total. at inference time, adapters merge into the original weights with zero overhead.

mathematically: instead of updating the full weight matrix $W \in \mathbb{R}^{d \times k}$, learn $\Delta W = BA$ where $B \in \mathbb{R}^{d \times r}$ and $A \in \mathbb{R}^{r \times k}$ with rank $r \ll \min(d,k)$.

### QLoRA

LoRA applied to a quantized base model (4-bit precision). enables fine-tuning 70B models on a single GPU with 48GB VRAM. the base weights are frozen and compressed; only the low-rank adapters train in full precision.

### RLHF (Reinforcement Learning from Human Feedback)

a two-stage process: first train a reward model from human preference pairs (chosen vs rejected responses), then optimize the language model against this reward using PPO. this is how raw language models become assistants that follow instructions and refuse harmful requests.

### DPO (Direct Preference Optimization)

achieves the same result as RLHF without training a separate reward model. directly optimizes the language model on preference pairs using a contrastive loss. simpler pipeline, comparable results.

---

## data formats

supervised fine-tuning uses instruction-response pairs:

```json
{"instruction": "summarize the text", "input": "...", "output": "..."}
```

preference tuning uses ranked pairs:

```json
{"prompt": "...", "chosen": "good response", "rejected": "bad response"}
```

quality of data matters more than quantity. a few hundred clean, representative examples often outperform tens of thousands of noisy ones.

---

## catastrophic forgetting

the central risk of fine-tuning. as the model adapts to new data, it may lose capabilities learned during pre-training. the weight updates that improve performance on the target task can degrade performance elsewhere.

mitigations: low learning rate, short training duration, mixing general data into the fine-tuning set, LoRA (which preserves original weights entirely).

---

## fine-tuning vs RAG

fine-tuning changes the model itself — it alters behavior, style, and reasoning patterns. it does not reliably inject new factual knowledge because gradient descent compresses information lossy across all weights.

[[retrieval augmented generation]] injects knowledge at inference time by providing relevant documents in the [[context]] window. it is more reliable for facts and easier to update.

in practice the two combine: fine-tune for behavior and format, use RAG for knowledge.

---

## the compilation alternative

standard fine-tuning is approximate inversion: from outputs (text) recover the structure that produced them. the [[cybergraph]] offers a different path — compile [[transformer]] weights directly from explicit graph structure. compiled weights are provably optimal (Eckart-Young theorem), require no training, and every weight traces to specific [[cyberlinks]].

the feedback loop: compile from graph, fine-tune on text to surface implicit associations, extract new links back into the graph, repeat. each cycle reduces approximation error. see [[transformer]] for the full derivation.

---

see [[transformer]] for the architecture being fine-tuned. see [[neural networks]] for foundations. see [[learning]] for the general principle. see [[llm]] for the models this technique primarily applies to.