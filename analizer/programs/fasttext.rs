//! fasttext — language identification
//! For: glotlid.
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor};

pub fn forward(text: &str, cfg: &Config, w: &Tensor) -> Vec<(u32, f32)> {
    let a = &cfg.architecture;

    // character n-gram hashing
    let ngrams = hash_ngrams(text, 2, 6, a.vocab_size);

    // embedding lookup + average
    let mut h = Tensor::zeros(a.hidden_size);
    for id in &ngrams {
        h = h.add(&w.row("model.embedding.weight", *id));
    }
    h = h.scale(1.0 / ngrams.len() as f32);

    // hidden layer
    h = w.linear(&h, "model.hidden.weight", a.hidden_size);

    // output — softmax over languages
    let logits = w.linear(&h, "model.output.weight", a.num_labels);
    logits.softmax().topk(5)
}

fn hash_ngrams(text: &str, min_n: usize, max_n: usize, bucket_size: usize) -> Vec<u32> {
    let mut result = Vec::new();
    let bounded = format!("<{text}>");
    let chars: Vec<char> = bounded.chars().collect();
    for n in min_n..=max_n {
        for window in chars.windows(n) {
            let s: String = window.iter().collect();
            let hash = fnv_hash(&s) % bucket_size as u32;
            result.push(hash);
        }
    }
    result
}

fn fnv_hash(s: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in s.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    h
}
