//! Vision-language model — image understanding + text generation
//! One program for: moondream, qwen-vl.
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor, Tokenizer};

pub fn forward(image: &Tensor, prompt: &str, cfg: &Config, w: &Tensor, tok: &Tokenizer) -> String {
    let a = &cfg.architecture;

    // vision encoder — ViT
    let patches = image.patchify(14);
    let mut v = w.linear(&patches, "model.vision.patch_embed.weight", a.hidden_size);
    v = v.add(&w.get("model.vision.pos_embed"));

    for i in 0..a.vision_layers {
        v = w.layernorm(&v, &format!("model.vision.blocks.{i}.norm1.weight"),
                        &format!("model.vision.blocks.{i}.norm1.bias"));
        let q = w.linear(&v, &format!("model.vision.blocks.{i}.attn.qkv.weight"), a.hidden_size * 3);
        let att = Tensor::self_attention(&q, a.num_attention_heads, a.head_dim);
        v = v.add(&w.linear(&att, &format!("model.vision.blocks.{i}.attn.proj.weight"), a.hidden_size));

        v = w.layernorm(&v, &format!("model.vision.blocks.{i}.norm2.weight"),
                        &format!("model.vision.blocks.{i}.norm2.bias"));
        let ff = w.linear(&v, &format!("model.vision.blocks.{i}.mlp.fc1.weight"), a.intermediate_size).gelu();
        v = v.add(&w.linear(&ff, &format!("model.vision.blocks.{i}.mlp.fc2.weight"), a.hidden_size));
    }

    // project vision → text space
    let visual_tokens = w.linear(&v, "model.vision_proj.weight", a.hidden_size);

    // text decoder with visual prefix
    let text_tokens = tok.encode(prompt);
    let mut h = w.embed("model.embed_tokens.weight", &text_tokens, a.hidden_size);
    h = visual_tokens.concat(&h);

    for i in 0..a.num_hidden_layers {
        h = w.rmsnorm(&h, &format!("model.layers.{i}.input_layernorm.weight"), a.rms_norm_eps);

        let q = w.linear(&h, &format!("model.layers.{i}.self_attn.q_proj.weight"), a.num_attention_heads * a.head_dim);
        let k = w.linear(&h, &format!("model.layers.{i}.self_attn.k_proj.weight"), a.num_key_value_heads * a.head_dim);
        let v = w.linear(&h, &format!("model.layers.{i}.self_attn.v_proj.weight"), a.num_key_value_heads * a.head_dim);

        let att = Tensor::flash_attention(&q, &k, &v, a.num_attention_heads, a.num_key_value_heads, a.head_dim);
        h = h.add(&w.linear(&att, &format!("model.layers.{i}.self_attn.o_proj.weight"), a.hidden_size));

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
