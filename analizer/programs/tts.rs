//! Text-to-speech — VITS / XTTS voice synthesis
//! One program for: piper, xtts-v2.
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor, Tokenizer};

pub fn forward(text: &str, cfg: &Config, w: &Tensor, tok: &Tokenizer) -> Vec<f32> {
    let a = &cfg.architecture;

    let tokens = tok.encode(text);

    // text encoder
    let mut h = w.embed("model.text_encoder.emb.weight", &tokens, a.hidden_size);

    for i in 0..a.text_encoder_layers {
        h = w.layernorm(&h, &format!("model.text_encoder.encoder.attn_layers.{i}.norm.weight"),
                        &format!("model.text_encoder.encoder.attn_layers.{i}.norm.bias"));
        let q = w.linear(&h, &format!("model.text_encoder.encoder.attn_layers.{i}.attn.q.weight"), a.hidden_size);
        let k = w.linear(&h, &format!("model.text_encoder.encoder.attn_layers.{i}.attn.k.weight"), a.hidden_size);
        let v = w.linear(&h, &format!("model.text_encoder.encoder.attn_layers.{i}.attn.v.weight"), a.hidden_size);
        h = h.add(&Tensor::attention(&q, &k, &v, a.num_attention_heads, a.head_dim));

        let ff = w.linear(&h, &format!("model.text_encoder.encoder.ffn_layers.{i}.fc1.weight"), a.intermediate_size).relu();
        h = h.add(&w.linear(&ff, &format!("model.text_encoder.encoder.ffn_layers.{i}.fc2.weight"), a.hidden_size));
    }

    // duration predictor → alignment
    let durations = w.linear(&h, "model.duration_predictor.linear.weight", 1).exp();
    let aligned = h.expand_by_durations(&durations);

    // flow decoder — inverse affine coupling
    let mut z = Tensor::randn(aligned.len(), a.hidden_size);
    for i in (0..a.flow_layers).rev() {
        let cond = w.conv1d(&aligned, &format!("model.flow.flows.{i}.enc.weight"), a.hidden_size);
        z = z.affine_coupling_inverse(&cond, &format!("model.flow.flows.{i}"), w);
    }

    // HiFi-GAN vocoder → waveform
    let mut audio = z;
    for i in 0..a.upsample_layers {
        audio = w.conv_transpose1d(&audio, &format!("model.vocoder.ups.{i}.weight"), a.upsample_channels).leaky_relu();
        for j in 0..3 {
            let r = w.conv1d(&audio, &format!("model.vocoder.resblocks.{i}.convs1.{j}.weight"), a.upsample_channels).leaky_relu();
            audio = audio.add(&w.conv1d(&r, &format!("model.vocoder.resblocks.{i}.convs2.{j}.weight"), a.upsample_channels));
        }
    }

    w.conv1d(&audio, "model.vocoder.conv_post.weight", 1).tanh().to_vec()
}
