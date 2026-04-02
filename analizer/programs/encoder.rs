//! Transformer encoder — classification and embeddings
//! One program for: deberta, modernbert, granite, jina.
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor, Tokenizer};

pub fn forward(input: &str, cfg: &Config, w: &Tensor, tok: &Tokenizer) -> Tensor {
    let a = &cfg.architecture;

    let tokens = tok.encode(input);
    let mut h = w.embed("model.embeddings.word_embeddings.weight", &tokens, a.hidden_size);
    h = h.add(&w.embed("model.embeddings.position_embeddings.weight", &tokens, a.hidden_size));
    h = w.layernorm(&h, "model.embeddings.LayerNorm.weight", "model.embeddings.LayerNorm.bias");

    for i in 0..a.num_hidden_layers {
        let q = w.linear(&h, &format!("model.encoder.layer.{i}.attention.self.query.weight"), a.hidden_size);
        let k = w.linear(&h, &format!("model.encoder.layer.{i}.attention.self.key.weight"), a.hidden_size);
        let v = w.linear(&h, &format!("model.encoder.layer.{i}.attention.self.value.weight"), a.hidden_size);

        let att = Tensor::attention(&q, &k, &v, a.num_attention_heads, a.head_dim);
        let att = w.linear(&att, &format!("model.encoder.layer.{i}.attention.output.dense.weight"), a.hidden_size);
        h = w.layernorm(&h.add(&att), &format!("model.encoder.layer.{i}.attention.output.LayerNorm.weight"),
                        &format!("model.encoder.layer.{i}.attention.output.LayerNorm.bias"));

        let ff = w.linear(&h, &format!("model.encoder.layer.{i}.intermediate.dense.weight"), a.intermediate_size).gelu();
        let ff = w.linear(&ff, &format!("model.encoder.layer.{i}.output.dense.weight"), a.hidden_size);
        h = w.layernorm(&h.add(&ff), &format!("model.encoder.layer.{i}.output.LayerNorm.weight"),
                        &format!("model.encoder.layer.{i}.output.LayerNorm.bias"));
    }

    // pool CLS token → classification or embedding
    h.select(0)
}
