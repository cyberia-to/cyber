//! BEATs — audio event classification
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor};

pub fn forward(audio: &[f32], cfg: &Config, w: &Tensor) -> Vec<(u32, f32)> {
    let a = &cfg.architecture;

    // patch embedding from raw audio
    let mut h = w.conv1d_patch(audio, "model.patch_embedding.weight", a.hidden_size, a.input_sample_rate);
    h = w.layernorm(&h, "model.layer_norm.weight", "model.layer_norm.bias");

    // transformer encoder
    for i in 0..a.num_hidden_layers {
        let residual = h.clone();
        h = w.layernorm(&h, &format!("model.encoder.layers.{i}.attention.layer_norm.weight"),
                        &format!("model.encoder.layers.{i}.attention.layer_norm.bias"));

        let q = w.linear(&h, &format!("model.encoder.layers.{i}.attention.self.query.weight"), a.hidden_size);
        let k = w.linear(&h, &format!("model.encoder.layers.{i}.attention.self.key.weight"), a.hidden_size);
        let v = w.linear(&h, &format!("model.encoder.layers.{i}.attention.self.value.weight"), a.hidden_size);

        let att = Tensor::attention(&q, &k, &v, a.num_attention_heads, a.head_dim);
        h = residual.add(&w.linear(&att, &format!("model.encoder.layers.{i}.attention.output.dense.weight"), a.hidden_size));

        let residual = h.clone();
        h = w.layernorm(&h, &format!("model.encoder.layers.{i}.feed_forward.layer_norm.weight"),
                        &format!("model.encoder.layers.{i}.feed_forward.layer_norm.bias"));
        let ff = w.linear(&h, &format!("model.encoder.layers.{i}.feed_forward.intermediate.weight"), a.intermediate_size).gelu();
        h = residual.add(&w.linear(&ff, &format!("model.encoder.layers.{i}.feed_forward.output.weight"), a.hidden_size));
    }

    // classification head
    let pooled = h.mean_pool();
    let logits = w.linear(&pooled, "model.classifier.weight", a.num_labels).sigmoid();

    // top-k predictions
    logits.topk(10)
}
