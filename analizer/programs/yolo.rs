//! YOLOv11 — object detection
//! Reads all parameters from config.

use cyb::nn::{Config, Tensor};

pub struct Detection {
    pub class: u32,
    pub confidence: f32,
    pub x: f32, pub y: f32, pub w: f32, pub h: f32,
}

pub fn forward(image: &Tensor, cfg: &Config, w: &Tensor) -> Vec<Detection> {
    let a = &cfg.architecture;

    // backbone — CSPDarknet with C3k2 blocks
    let mut x = w.conv2d(image, "model.0.conv.weight", 16, 3, 2).silu();  // P1
    x = w.conv2d(&x, "model.1.conv.weight", 32, 3, 2).silu();             // P2
    let p3 = c3k2(w, &x, "model.2", 64);                                   // P3
    let p4 = c3k2(w, &w.conv2d(&p3, "model.3.conv.weight", 64, 3, 2).silu(), "model.4", 128); // P4
    let p5 = c3k2(w, &w.conv2d(&p4, "model.5.conv.weight", 128, 3, 2).silu(), "model.6", 256); // P5

    // neck — FPN + PAN
    let up5 = p5.upsample(2);
    let n4 = c3k2(w, &p4.concat(&up5), "model.8", 128);
    let up4 = n4.upsample(2);
    let n3 = c3k2(w, &p3.concat(&up4), "model.10", 64);

    let d4 = c3k2(w, &n3.concat(&w.conv2d(&n3, "model.11.conv.weight", 64, 3, 2).silu()), "model.12", 128);
    let d5 = c3k2(w, &d4.concat(&w.conv2d(&d4, "model.13.conv.weight", 128, 3, 2).silu()), "model.14", 256);

    // detection head — multi-scale
    let out3 = detect_head(w, &n3, "model.15", a.num_classes);
    let out4 = detect_head(w, &d4, "model.15", a.num_classes);
    let out5 = detect_head(w, &d5, "model.15", a.num_classes);

    nms(out3.concat(&out4).concat(&out5), 0.5)
}

fn c3k2(w: &Tensor, x: &Tensor, prefix: &str, channels: usize) -> Tensor {
    let a = w.conv2d(x, &format!("{prefix}.cv1.conv.weight"), channels, 1, 1).silu();
    let b = w.conv2d(&a, &format!("{prefix}.m.0.cv1.conv.weight"), channels / 2, 3, 1).silu();
    let b = w.conv2d(&b, &format!("{prefix}.m.0.cv2.conv.weight"), channels / 2, 3, 1).silu();
    w.conv2d(&a.concat(&b), &format!("{prefix}.cv2.conv.weight"), channels, 1, 1).silu()
}

fn detect_head(w: &Tensor, x: &Tensor, prefix: &str, num_classes: usize) -> Tensor {
    let bbox = w.conv2d(x, &format!("{prefix}.dfl.conv.weight"), 64, 1, 1);
    let cls = w.conv2d(x, &format!("{prefix}.cls.conv.weight"), num_classes, 1, 1).sigmoid();
    bbox.concat(&cls)
}

fn nms(detections: Tensor, threshold: f32) -> Vec<Detection> {
    detections.non_max_suppression(threshold)
}
