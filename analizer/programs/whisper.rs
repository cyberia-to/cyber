//! Whisper — encoder-decoder speech recognition
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor};

pub fn forward(audio: &[f32], cfg: &Config, w: &Tensor) -> Vec<u32> {
    let a = &cfg.architecture;

    // mel spectrogram → encoder
    let mel = Tensor::mel_spectrogram(audio, a.num_mels);
    let mut h = w.conv1d(&mel, "model.encoder.conv1.weight", a.hidden_size);
    h = w.conv1d(&h, "model.encoder.conv2.weight", a.hidden_size).gelu();
    h = h.add(&w.get("model.encoder.embed_positions.weight"));

    for i in 0..a.num_hidden_layers {
        h = w.layernorm(&h, &format!("model.encoder.layers.{i}.self_attn_layer_norm.weight"),
                        &format!("model.encoder.layers.{i}.self_attn_layer_norm.bias"));
        let q = w.linear(&h, &format!("model.encoder.layers.{i}.self_attn.q_proj.weight"), a.hidden_size);
        let k = w.linear(&h, &format!("model.encoder.layers.{i}.self_attn.k_proj.weight"), a.hidden_size);
        let v = w.linear(&h, &format!("model.encoder.layers.{i}.self_attn.v_proj.weight"), a.hidden_size);
        let att = Tensor::attention(&q, &k, &v, a.num_attention_heads, a.head_dim);
        h = h.add(&w.linear(&att, &format!("model.encoder.layers.{i}.self_attn.out_proj.weight"), a.hidden_size));

        h = w.layernorm(&h, &format!("model.encoder.layers.{i}.final_layer_norm.weight"),
                        &format!("model.encoder.layers.{i}.final_layer_norm.bias"));
        let ff = w.linear(&h, &format!("model.encoder.layers.{i}.fc1.weight"), a.intermediate_size).gelu();
        h = h.add(&w.linear(&ff, &format!("model.encoder.layers.{i}.fc2.weight"), a.hidden_size));
    }

    h = w.layernorm(&h, "model.encoder.layer_norm.weight", "model.encoder.layer_norm.bias");

    // decoder — autoregressive token generation
    let mut tokens = vec![50258]; // <|startoftranscript|>
    let enc = h;

    loop {
        let mut d = w.embed("model.decoder.embed_tokens.weight", &tokens, a.hidden_size);
        d = d.add(&w.get("model.decoder.embed_positions.weight"));

        for i in 0..a.num_hidden_layers {
            d = w.layernorm(&d, &format!("model.decoder.layers.{i}.self_attn_layer_norm.weight"),
                            &format!("model.decoder.layers.{i}.self_attn_layer_norm.bias"));
            let q = w.linear(&d, &format!("model.decoder.layers.{i}.self_attn.q_proj.weight"), a.hidden_size);
            let k = w.linear(&d, &format!("model.decoder.layers.{i}.self_attn.k_proj.weight"), a.hidden_size);
            let v = w.linear(&d, &format!("model.decoder.layers.{i}.self_attn.v_proj.weight"), a.hidden_size);
            d = d.add(&Tensor::causal_attention(&q, &k, &v, a.num_attention_heads, a.head_dim));

            d = w.layernorm(&d, &format!("model.decoder.layers.{i}.encoder_attn_layer_norm.weight"),
                            &format!("model.decoder.layers.{i}.encoder_attn_layer_norm.bias"));
            let q = w.linear(&d, &format!("model.decoder.layers.{i}.encoder_attn.q_proj.weight"), a.hidden_size);
            let k = w.linear(&enc, &format!("model.decoder.layers.{i}.encoder_attn.k_proj.weight"), a.hidden_size);
            let v = w.linear(&enc, &format!("model.decoder.layers.{i}.encoder_attn.v_proj.weight"), a.hidden_size);
            d = d.add(&Tensor::attention(&q, &k, &v, a.num_attention_heads, a.head_dim));

            d = w.layernorm(&d, &format!("model.decoder.layers.{i}.final_layer_norm.weight"),
                            &format!("model.decoder.layers.{i}.final_layer_norm.bias"));
            let ff = w.linear(&d, &format!("model.decoder.layers.{i}.fc1.weight"), a.intermediate_size).gelu();
            d = d.add(&w.linear(&ff, &format!("model.decoder.layers.{i}.fc2.weight"), a.hidden_size));
        }

        d = w.layernorm(&d, "model.decoder.layer_norm.weight", "model.decoder.layer_norm.bias");
        let logits = w.linear(&d, "model.decoder.embed_tokens.weight", a.vocab_size);
        let next = logits.last().argmax();

        if next == 50257 { break; } // <|endoftext|>
        tokens.push(next);
    }

    tokens
}
