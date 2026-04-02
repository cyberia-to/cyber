//! Video generation — diffusion transformer
//! For: wan2.2.
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor, Tokenizer};

pub fn forward(prompt: &str, cfg: &Config, w: &Tensor, tok: &Tokenizer) -> Tensor {
    let a = &cfg.architecture;

    // text encoding (CLIP)
    let tokens = tok.encode(prompt);
    let mut cond = w.embed("model.text_encoder.token_embedding.weight", &tokens, a.text_hidden_size);
    for i in 0..a.text_encoder_layers {
        cond = w.layernorm(&cond, &format!("model.text_encoder.transformer.resblocks.{i}.ln_1.weight"),
                           &format!("model.text_encoder.transformer.resblocks.{i}.ln_1.bias"));
        let qkv = w.linear(&cond, &format!("model.text_encoder.transformer.resblocks.{i}.attn.in_proj_weight"), a.text_hidden_size * 3);
        cond = cond.add(&Tensor::self_attention(&qkv, a.text_num_heads, a.text_head_dim));

        cond = w.layernorm(&cond, &format!("model.text_encoder.transformer.resblocks.{i}.ln_2.weight"),
                           &format!("model.text_encoder.transformer.resblocks.{i}.ln_2.bias"));
        let ff = w.linear(&cond, &format!("model.text_encoder.transformer.resblocks.{i}.mlp.c_fc.weight"), a.text_hidden_size * 4).gelu();
        cond = cond.add(&w.linear(&ff, &format!("model.text_encoder.transformer.resblocks.{i}.mlp.c_proj.weight"), a.text_hidden_size));
    }

    // diffusion — denoise from noise
    let mut x = Tensor::randn_video(a.num_frames, a.height, a.width, a.latent_channels);

    for step in (0..a.num_inference_steps).rev() {
        let t = Tensor::timestep(step, a.num_inference_steps);

        // DiT blocks
        let mut h = w.linear(&x.flatten_patches(), "model.dit.patch_embed.weight", a.hidden_size);
        h = h.add(&w.get("model.dit.pos_embed"));

        for i in 0..a.num_hidden_layers {
            let scale = w.linear(&t, &format!("model.dit.blocks.{i}.adaLN.weight"), a.hidden_size * 6);
            h = h.adaptive_layernorm(&scale);

            let q = w.linear(&h, &format!("model.dit.blocks.{i}.attn.q.weight"), a.hidden_size);
            let k = w.linear(&h, &format!("model.dit.blocks.{i}.attn.k.weight"), a.hidden_size);
            let v = w.linear(&h, &format!("model.dit.blocks.{i}.attn.v.weight"), a.hidden_size);
            h = h.add(&Tensor::attention(&q, &k, &v, a.num_attention_heads, a.head_dim));

            // cross-attention with text
            let q = w.linear(&h, &format!("model.dit.blocks.{i}.cross_attn.q.weight"), a.hidden_size);
            let k = w.linear(&cond, &format!("model.dit.blocks.{i}.cross_attn.k.weight"), a.hidden_size);
            let v = w.linear(&cond, &format!("model.dit.blocks.{i}.cross_attn.v.weight"), a.hidden_size);
            h = h.add(&Tensor::attention(&q, &k, &v, a.num_attention_heads, a.head_dim));

            let ff = w.linear(&h, &format!("model.dit.blocks.{i}.mlp.fc1.weight"), a.intermediate_size).gelu();
            h = h.add(&w.linear(&ff, &format!("model.dit.blocks.{i}.mlp.fc2.weight"), a.hidden_size));
        }

        let noise_pred = w.linear(&h, "model.dit.final_layer.weight", a.latent_channels).unflatten_patches();
        x = x.ddim_step(&noise_pred, step, a.num_inference_steps);
    }

    // VAE decode → pixel space
    vae_decode(&x, w, a)
}

fn vae_decode(latent: &Tensor, w: &Tensor, a: &Architecture) -> Tensor {
    let mut h = w.conv3d(latent, "model.vae.decoder.conv_in.weight", a.vae_channels);
    for i in 0..a.vae_layers {
        let r = w.conv3d(&h, &format!("model.vae.decoder.up_blocks.{i}.resnets.0.conv1.weight"), a.vae_channels).silu();
        h = h.add(&w.conv3d(&r, &format!("model.vae.decoder.up_blocks.{i}.resnets.0.conv2.weight"), a.vae_channels));
        h = h.upsample3d(2);
    }
    w.conv3d(&h.silu(), "model.vae.decoder.conv_out.weight", 3)
}
