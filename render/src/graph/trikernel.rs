use crate::parser::PageId;
use std::collections::HashMap;

use super::PageStore;

/// Tri-kernel parameters
const ALPHA: f64 = 0.15; // diffusion teleport probability
const MU: f64 = 1.0; // spring screening strength
const TAU: f64 = 1.0; // heat kernel temperature
const LAMBDA_D: f64 = 0.5; // diffusion blend weight
const LAMBDA_S: f64 = 0.3; // springs blend weight
const LAMBDA_H: f64 = 0.2; // heat kernel blend weight
const MAX_ITER: usize = 50;
const CONVERGENCE: f64 = 1e-6;
const HEAT_SUBSTEPS: usize = 20;

/// Compute tri-kernel focus distribution π for all pages.
/// π = λ_d·D(stake) + λ_s·S(stake) + λ_h·H_τ(stake)
///
/// - D: personalized PageRank with stake-weighted teleport
/// - S: screened Laplacian inverse applied to stake
/// - H_τ: heat kernel smoothing of stake
pub fn compute_trikernel(store: &PageStore) -> HashMap<PageId, f64> {
    let page_ids: Vec<&PageId> = store.pages.keys().collect();
    let n = page_ids.len();
    if n == 0 {
        return HashMap::new();
    }

    let id_to_idx: HashMap<&PageId, usize> = page_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i))
        .collect();

    // Build adjacency: in_links[i] = vec of (source_idx, source_out_degree)
    let mut in_links: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    // Also build adjacency list for undirected operations (springs, heat)
    let mut neighbors: Vec<Vec<usize>> = vec![vec![]; n];
    let mut degree: Vec<usize> = vec![0; n];

    for (source_id, targets) in &store.forward_links {
        let Some(&src_idx) = id_to_idx.get(source_id) else {
            continue;
        };
        let valid_targets: Vec<usize> = targets
            .iter()
            .filter_map(|t| id_to_idx.get(t).copied())
            .collect();
        let out_degree = valid_targets.len();
        if out_degree == 0 {
            continue;
        }
        for tgt_idx in valid_targets {
            in_links[tgt_idx].push((src_idx, out_degree));
            // Undirected neighbors (add both directions)
            if !neighbors[src_idx].contains(&tgt_idx) {
                neighbors[src_idx].push(tgt_idx);
            }
            if !neighbors[tgt_idx].contains(&src_idx) {
                neighbors[tgt_idx].push(src_idx);
            }
        }
    }

    // Compute undirected degree
    for i in 0..n {
        degree[i] = neighbors[i].len();
    }

    // Build stake vector (normalized to sum to 1)
    let stake_raw: Vec<f64> = page_ids
        .iter()
        .map(|id| {
            store
                .pages
                .get(*id)
                .and_then(|p| p.meta.stake)
                .unwrap_or(1) as f64
        })
        .collect();
    let stake_sum: f64 = stake_raw.iter().sum();
    let stake: Vec<f64> = stake_raw.iter().map(|s| s / stake_sum).collect();

    // Identify dangling nodes (no outgoing links to valid pages)
    let has_outlinks: Vec<bool> = (0..n)
        .map(|i| {
            let id = page_ids[i];
            store
                .forward_links
                .get(id)
                .map(|targets| targets.iter().any(|t| id_to_idx.contains_key(t)))
                .unwrap_or(false)
        })
        .collect();

    // === Operator 1: Diffusion (Personalized PageRank with stake teleport) ===
    let diffusion = compute_diffusion(n, &stake, &in_links, &has_outlinks);

    // === Operator 2: Springs (Screened Laplacian inverse) ===
    let springs = compute_springs(n, &stake, &neighbors, &degree);

    // === Operator 3: Heat Kernel ===
    let heat = compute_heat(n, &stake, &neighbors, &degree);

    // === Blend: π = λ_d·D + λ_s·S + λ_h·H ===
    let mut focus = vec![0.0; n];
    for i in 0..n {
        focus[i] = LAMBDA_D * diffusion[i] + LAMBDA_S * springs[i] + LAMBDA_H * heat[i];
    }

    // Normalize to sum to 1
    let focus_sum: f64 = focus.iter().sum();
    if focus_sum > 0.0 {
        for f in &mut focus {
            *f /= focus_sum;
        }
    }

    // Map back to PageId
    page_ids
        .into_iter()
        .enumerate()
        .map(|(i, id)| (id.clone(), focus[i]))
        .collect()
}

/// Personalized PageRank with stake-weighted teleport.
/// π_new[i] = α·stake[i] + (1-α)·Σ(π[j]/out_deg(j)) for j→i
/// Dangling nodes distribute their mass proportional to stake.
fn compute_diffusion(
    n: usize,
    stake: &[f64],
    in_links: &[Vec<(usize, usize)>],
    has_outlinks: &[bool],
) -> Vec<f64> {
    let mut rank: Vec<f64> = stake.to_vec();

    for _ in 0..MAX_ITER {
        let dangling_sum: f64 = rank
            .iter()
            .enumerate()
            .filter(|(i, _)| !has_outlinks[*i])
            .map(|(_, r)| r)
            .sum();

        let mut new_rank = vec![0.0; n];
        for i in 0..n {
            // Teleport + dangling redistribution (both proportional to stake)
            new_rank[i] = ALPHA * stake[i] + (1.0 - ALPHA) * dangling_sum * stake[i];
            // Incoming link contributions
            for &(src_idx, out_degree) in &in_links[i] {
                new_rank[i] += (1.0 - ALPHA) * rank[src_idx] / out_degree as f64;
            }
        }

        let delta: f64 = new_rank
            .iter()
            .zip(rank.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        rank = new_rank;
        if delta < CONVERGENCE {
            break;
        }
    }

    rank
}

/// Screened Laplacian inverse: solve (μI + L)x = stake via Jacobi iteration.
/// L is the unnormalized graph Laplacian: L_ii = degree(i), L_ij = -1 if connected.
/// Jacobi: x_new[i] = (stake[i] + Σ_{j∈neighbors} x[j]) / (μ + degree[i])
fn compute_springs(
    n: usize,
    stake: &[f64],
    neighbors: &[Vec<usize>],
    degree: &[usize],
) -> Vec<f64> {
    let mut x: Vec<f64> = stake.to_vec();

    for _ in 0..MAX_ITER {
        let mut x_new = vec![0.0; n];
        for i in 0..n {
            let neighbor_sum: f64 = neighbors[i].iter().map(|&j| x[j]).sum();
            x_new[i] = (stake[i] + neighbor_sum) / (MU + degree[i] as f64);
        }

        let delta: f64 = x_new
            .iter()
            .zip(x.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        x = x_new;
        if delta < CONVERGENCE {
            break;
        }
    }

    // Normalize to sum to 1
    let sum: f64 = x.iter().sum();
    if sum > 0.0 {
        for v in &mut x {
            *v /= sum;
        }
    }
    x
}

/// Heat kernel: approximate e^{-τL} applied to stake via Euler substeps.
/// Each substep: s_new[i] = s[i] + (τ/k)·(Σ_{j∈neighbors} s[j]/deg(j) - s[i])
/// This is a normalized Laplacian diffusion.
fn compute_heat(
    n: usize,
    stake: &[f64],
    neighbors: &[Vec<usize>],
    degree: &[usize],
) -> Vec<f64> {
    let mut s: Vec<f64> = stake.to_vec();
    let dt = TAU / HEAT_SUBSTEPS as f64;

    for _ in 0..HEAT_SUBSTEPS {
        let mut s_new = vec![0.0; n];
        for i in 0..n {
            let neighbor_sum: f64 = neighbors[i]
                .iter()
                .map(|&j| {
                    if degree[j] > 0 {
                        s[j] / degree[j] as f64
                    } else {
                        0.0
                    }
                })
                .sum();
            s_new[i] = s[i] + dt * (neighbor_sum - s[i]);
        }
        s = s_new;
    }

    // Normalize to sum to 1
    let sum: f64 = s.iter().sum();
    if sum > 0.0 {
        for v in &mut s {
            *v /= sum;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::build_graph;
    use crate::parser::{slugify_page_name, PageKind, PageMeta, ParsedPage};
    use std::path::PathBuf;

    fn make_page(name: &str, links: Vec<&str>, stake_val: u64) -> ParsedPage {
        ParsedPage {
            id: slugify_page_name(name),
            meta: PageMeta {
                title: name.to_string(),
                properties: HashMap::new(),
                tags: vec![],
                public: Some(true),
                aliases: vec![],
                date: None,
                icon: None,
                menu_order: None,
                stake: Some(stake_val),
            },
            kind: PageKind::Page,
            source_path: PathBuf::from(format!("pages/{}.md", name)),
            namespace: None,
            subgraph: None,
            content_md: String::new(),
            outgoing_links: links.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_trikernel_sums_to_one() {
        let pages = vec![
            make_page("X", vec!["Y", "Z"], 100),
            make_page("Y", vec!["Z"], 50),
            make_page("Z", vec!["X"], 200),
        ];
        let store = build_graph(pages).unwrap();
        let focus = compute_trikernel(&store);
        let total: f64 = focus.values().sum();
        assert!((total - 1.0).abs() < 0.01, "Focus must sum to 1, got {}", total);
    }

    #[test]
    fn test_trikernel_stake_influence() {
        // D has 10x more stake than others but fewer links
        let pages = vec![
            make_page("A", vec!["B"], 100),
            make_page("B", vec!["C"], 100),
            make_page("C", vec!["A"], 100),
            make_page("D", vec!["A"], 1000),
        ];
        let store = build_graph(pages).unwrap();
        let focus = compute_trikernel(&store);

        let d_id = slugify_page_name("D");
        let b_id = slugify_page_name("B");

        // D should have higher focus than B due to higher stake
        assert!(
            focus[&d_id] > focus[&b_id],
            "Higher-stake node D ({}) should have more focus than B ({})",
            focus[&d_id],
            focus[&b_id]
        );
    }

    #[test]
    fn test_trikernel_link_structure_matters() {
        // A gets links from both C and D
        let pages = vec![
            make_page("A", vec!["B"], 100),
            make_page("B", vec!["C"], 100),
            make_page("C", vec!["A"], 100),
            make_page("D", vec!["A"], 100),
        ];
        let store = build_graph(pages).unwrap();
        let focus = compute_trikernel(&store);

        let a_id = slugify_page_name("A");
        let b_id = slugify_page_name("B");

        // A should have highest focus (gets link from C and D)
        assert!(
            focus[&a_id] > focus[&b_id],
            "Well-linked A ({}) should have more focus than B ({})",
            focus[&a_id],
            focus[&b_id]
        );
    }
}
