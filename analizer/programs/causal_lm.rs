//! Causal language model — transformer decoder
//! One program for any size: qwen, deepseek, smollm, nuextract, mimo.
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor, Tokenizer};

pub fn forward(input: &str, cfg: &Config, w: &Tensor, tok: &Tokenizer) -> String {
    let a = &cfg.architecture;

    let tokens = tok.encode(input);
    let mut h = w.embed("model.embed_tokens.weight", &tokens, a.hidden_size);

    for i in 0..a.num_hidden_layers {
        h = w.rmsnorm(&h, &format!("model.layers.{i}.input_layernorm.weight"), a.rms_norm_eps);

        let q = w.linear(&h, &format!("model.layers.{i}.self_attn.q_proj.weight"), a.num_attention_heads * a.head_dim);
        let k = w.linear(&h, &format!("model.layers.{i}.self_attn.k_proj.weight"), a.num_key_value_heads * a.head_dim);
        let v = w.linear(&h, &format!("model.layers.{i}.self_attn.v_proj.weight"), a.num_key_value_heads * a.head_dim);

        let att = Tensor::flash_attention(&q, &k, &v, a.num_attention_heads, a.num_key_value_heads, a.head_dim);
        let att = w.linear(&att, &format!("model.layers.{i}.self_attn.o_proj.weight"), a.hidden_size);
        h = h.add(&att);

        h = w.rmsnorm(&h, &format!("model.layers.{i}.post_attention_layernorm.weight"), a.rms_norm_eps);

        let gate = w.linear(&h, &format!("model.layers.{i}.mlp.gate_proj.weight"), a.intermediate_size);
        let up = w.linear(&h, &format!("model.layers.{i}.mlp.up_proj.weight"), a.intermediate_size);
        h = h.add(&w.linear(&gate.silu().mul(&up), &format!("model.layers.{i}.mlp.down_proj.weight"), a.hidden_size));
    }

    h = w.rmsnorm(&h, "model.norm.weight", a.rms_norm_eps);
    let logits = w.linear(&h, "lm_head.weight", a.vocab_size);

    let s = &cfg.sampling;
    let token = logits.sample_top_p(s.top_p, s.temperature, s.scale);
    tok.decode(&[token])
}
