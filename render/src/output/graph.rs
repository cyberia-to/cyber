use crate::config::SiteConfig;
use crate::graph::PageStore;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Serialize)]
struct GraphData {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

#[derive(Serialize)]
struct GraphNode {
    id: String,
    title: String,
    url: String,
    tags: Vec<String>,
    #[serde(rename = "linkCount")]
    link_count: usize,
    #[serde(rename = "pageRank")]
    page_rank: f64,
    focus: f64,
    x: f64,
    y: f64,
}

#[derive(Serialize)]
struct GraphEdge {
    source: String,
    target: String,
}

pub fn generate_graph_data(
    store: &PageStore,
    config: &SiteConfig,
    output_dir: &Path,
) -> Result<()> {
    let public_ids: HashSet<&String> = store
        .public_pages(&config.content)
        .iter()
        .map(|p| &p.id)
        .collect();

    let mut nodes: Vec<GraphNode> = store
        .public_pages(&config.content)
        .iter()
        .map(|page| {
            let backlink_count = store
                .backlinks
                .get(&page.id)
                .map(|b| b.len())
                .unwrap_or(0);
            let forward_count = store
                .forward_links
                .get(&page.id)
                .map(|f| f.len())
                .unwrap_or(0);

            let pr = store.pagerank.get(&page.id).copied().unwrap_or(0.0);
            let fc = store.focus.get(&page.id).copied().unwrap_or(0.0);
            GraphNode {
                id: page.id.clone(),
                title: page.meta.title.clone(),
                url: format!("/{}", page.id),
                tags: page.meta.tags.clone(),
                link_count: backlink_count + forward_count,
                page_rank: (pr * 100000.0).round() / 100000.0,
                focus: (fc * 100000.0).round() / 100000.0,
                x: 0.0,
                y: 0.0,
            }
        })
        .collect();

    let mut edges = Vec::new();
    let mut seen_edges: HashSet<(String, String)> = HashSet::new();

    for (source_id, targets) in &store.forward_links {
        if !public_ids.contains(source_id) {
            continue;
        }
        for target_id in targets {
            if !public_ids.contains(target_id) {
                continue;
            }
            let edge_key = if source_id < target_id {
                (source_id.clone(), target_id.clone())
            } else {
                (target_id.clone(), source_id.clone())
            };
            if seen_edges.insert(edge_key) {
                edges.push(GraphEdge {
                    source: source_id.clone(),
                    target: target_id.clone(),
                });
            }
        }
    }

    // Pre-compute force-directed layout
    compute_layout(&mut nodes, &edges);

    let data = GraphData { nodes, edges };
    let json = serde_json::to_string(&data)?;

    // Write JSON for minimap/API use
    std::fs::write(output_dir.join("graph-data.json"), &json)?;

    // Write JS module for direct inclusion in graph page (avoids fetch)
    let static_dir = output_dir.join("static");
    std::fs::create_dir_all(&static_dir)?;
    std::fs::write(
        static_dir.join("graph-data.js"),
        format!("window.__GRAPH_DATA={};", json),
    )?;

    Ok(())
}

/// Barnes-Hut quadtree node for O(n log n) repulsive force approximation.
struct QuadNode {
    cx: f64,
    cy: f64,
    mass: f64,
    children: [Option<Box<QuadNode>>; 4],
    is_leaf: bool,
    index: usize, // only valid for leaves
}

impl QuadNode {
    fn new_leaf(x: f64, y: f64, idx: usize) -> Self {
        QuadNode {
            cx: x, cy: y, mass: 1.0,
            children: [None, None, None, None],
            is_leaf: true, index: idx,
        }
    }

    fn new_empty() -> Self {
        QuadNode {
            cx: 0.0, cy: 0.0, mass: 0.0,
            children: [None, None, None, None],
            is_leaf: false, index: 0,
        }
    }
}

fn build_quadtree(
    x: &[f64], y: &[f64],
    indices: &[usize],
    bx: f64, by: f64, bw: f64,
    depth: usize,
) -> Option<Box<QuadNode>> {
    if indices.is_empty() {
        return None;
    }
    if indices.len() == 1 {
        let i = indices[0];
        return Some(Box::new(QuadNode::new_leaf(x[i], y[i], i)));
    }
    // Max depth: treat remaining as single aggregate node
    if depth >= 40 {
        let mass = indices.len() as f64;
        let cx = indices.iter().map(|&i| x[i]).sum::<f64>() / mass;
        let cy = indices.iter().map(|&i| y[i]).sum::<f64>() / mass;
        let mut node = QuadNode::new_empty();
        node.cx = cx;
        node.cy = cy;
        node.mass = mass;
        return Some(Box::new(node));
    }

    let hw = bw / 2.0;
    let mx = bx + hw;
    let my = by + hw;

    let mut quadrants: [Vec<usize>; 4] = [vec![], vec![], vec![], vec![]];
    for &i in indices {
        let q = if x[i] < mx {
            if y[i] < my { 0 } else { 2 }
        } else {
            if y[i] < my { 1 } else { 3 }
        };
        quadrants[q].push(i);
    }

    let offsets = [(bx, by), (mx, by), (bx, my), (mx, my)];
    let mut node = QuadNode::new_empty();

    let mut total_x = 0.0;
    let mut total_y = 0.0;
    let mut total_mass = 0.0;

    for q in 0..4 {
        node.children[q] = build_quadtree(x, y, &quadrants[q], offsets[q].0, offsets[q].1, hw, depth + 1);
        if let Some(ref child) = node.children[q] {
            total_x += child.cx * child.mass;
            total_y += child.cy * child.mass;
            total_mass += child.mass;
        }
    }

    if total_mass > 0.0 {
        node.cx = total_x / total_mass;
        node.cy = total_y / total_mass;
        node.mass = total_mass;
    }

    Some(Box::new(node))
}

/// Apply Barnes-Hut repulsion (D3-style inverse-square: strength / dist²).
fn quadtree_repulsion(
    node: &QuadNode,
    px: f64, py: f64, pi: usize,
    strength: f64, theta_sq: f64, bw: f64,
    dvx: &mut f64, dvy: &mut f64,
) {
    if node.mass == 0.0 { return; }

    let ddx = node.cx - px;
    let ddy = node.cy - py;
    let dist_sq = (ddx * ddx + ddy * ddy).max(1.0);

    if node.is_leaf {
        if node.index == pi { return; }
        let w = strength / dist_sq;
        *dvx += ddx * w;
        *dvy += ddy * w;
        return;
    }

    // Barnes-Hut criterion
    if (bw * bw) / dist_sq < theta_sq {
        let w = strength * node.mass / dist_sq;
        *dvx += ddx * w;
        *dvy += ddy * w;
        return;
    }

    let hw = bw / 2.0;
    for q in 0..4 {
        if let Some(ref child) = node.children[q] {
            quadtree_repulsion(child, px, py, pi, strength, theta_sq, hw, dvx, dvy);
        }
    }
}

/// Quadtree-based collision detection: push overlapping nodes apart.
fn quadtree_collision(
    node: &QuadNode,
    px: f64, py: f64, pi: usize, ri: f64,
    radii: &[f64], x: &[f64], y: &[f64],
    max_r: f64,
    bx: f64, by: f64, bw: f64,
    dvx: &mut f64, dvy: &mut f64,
) {
    if node.mass == 0.0 { return; }

    // Quick reject: closest point in cell to (px,py) beyond collision range?
    let closest_x = px.clamp(bx, bx + bw);
    let closest_y = py.clamp(by, by + bw);
    let cdx = px - closest_x;
    let cdy = py - closest_y;
    if cdx * cdx + cdy * cdy > (ri + max_r + 1.0) * (ri + max_r + 1.0) {
        return;
    }

    if node.is_leaf {
        let j = node.index;
        if j == pi { return; }
        let ddx = px - x[j];
        let ddy = py - y[j];
        let dist_sq = ddx * ddx + ddy * ddy;
        let min_dist = ri + radii[j];
        if dist_sq < min_dist * min_dist {
            let dist = dist_sq.sqrt().max(0.001);
            let overlap = (min_dist - dist) / dist * 0.5;
            *dvx += ddx * overlap;
            *dvy += ddy * overlap;
        }
        return;
    }

    let hw = bw / 2.0;
    let offsets = [(bx, by), (bx + hw, by), (bx, by + hw), (bx + hw, by + hw)];
    for q in 0..4 {
        if let Some(ref child) = node.children[q] {
            quadtree_collision(child, px, py, pi, ri, radii, x, y, max_r, offsets[q].0, offsets[q].1, hw, dvx, dvy);
        }
    }
}

/// Compute force-directed layout using D3-style velocity Verlet with Barnes-Hut.
/// Includes charge repulsion, link attraction, centering, and collision forces.
fn compute_layout(nodes: &mut [GraphNode], edges: &[GraphEdge]) {
    use std::collections::HashMap;

    let n = nodes.len();
    if n == 0 {
        return;
    }

    let id_to_idx: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.id.as_str(), i))
        .collect();

    let edge_pairs: Vec<(usize, usize)> = edges
        .iter()
        .filter_map(|e| {
            let a = id_to_idx.get(e.source.as_str())?;
            let b = id_to_idx.get(e.target.as_str())?;
            Some((*a, *b))
        })
        .collect();

    // Compute node degrees for link bias
    let mut degree = vec![0usize; n];
    for &(a, b) in &edge_pairs {
        degree[a] += 1;
        degree[b] += 1;
    }

    // Collision radii in layout space (matches JS radiusScale mapped to layout units)
    // JS viewport ~1200px maps to ~8000 layout units → scale ~0.15
    // JS radius range [1, 18]px → layout radius [7, 120]
    let max_lc = nodes.iter().map(|n| n.link_count).max().unwrap_or(1) as f64;
    let radii: Vec<f64> = nodes.iter().map(|node| {
        let t = (node.link_count as f64 / max_lc).sqrt();
        7.0 + t * 113.0
    }).collect();
    let max_radius = radii.iter().cloned().fold(0.0_f64, f64::max);

    // Max PageRank for gravity scaling
    let max_pr = nodes.iter().map(|n| n.page_rank).fold(0.0_f64, f64::max).max(0.00001);

    // D3-style simulation parameters
    let charge = -30.0_f64;
    let link_distance = 30.0_f64;
    let link_strength = 0.3_f64;
    let velocity_decay = 0.4_f64;
    let alpha_decay = 0.023_f64;
    let theta_sq: f64 = 0.81; // 0.9²

    // Deterministic pseudo-random initialization (no visible patterns)
    let mut x = vec![0.0f64; n];
    let mut y = vec![0.0f64; n];
    let spread = (n as f64).sqrt() * 4.0;
    for i in 0..n {
        let hash = (i as u64).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let hx = ((hash >> 0) & 0xFFFF) as f64 / 65536.0 - 0.5;
        let hy = ((hash >> 16) & 0xFFFF) as f64 / 65536.0 - 0.5;
        x[i] = hx * spread;
        y[i] = hy * spread;
    }

    let mut vx = vec![0.0f64; n];
    let mut vy = vec![0.0f64; n];
    let mut alpha = 1.0_f64;
    let indices: Vec<usize> = (0..n).collect();

    for tick in 0..300 {
        alpha *= 1.0 - alpha_decay;
        if alpha < 0.001 {
            break;
        }

        // Compute bounding box for quadtree
        let min_x = x.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_x = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_y = y.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let bw = (max_x - min_x).max(max_y - min_y).max(1.0) * 1.01;
        let bx = (min_x + max_x - bw) / 2.0;
        let by = (min_y + max_y - bw) / 2.0;

        if let Some(root) = build_quadtree(&x, &y, &indices, bx, by, bw, 0) {
            // Charge (repulsion) via Barnes-Hut
            let strength = charge * alpha;
            for i in 0..n {
                quadtree_repulsion(
                    &root, x[i], y[i], i,
                    strength, theta_sq, bw,
                    &mut vx[i], &mut vy[i],
                );
            }

            // Collision — push overlapping nodes apart (every 3rd tick for performance)
            if tick % 3 == 0 {
            let collision_str = 0.7_f64;
            for i in 0..n {
                let mut cvx = 0.0;
                let mut cvy = 0.0;
                quadtree_collision(
                    &root, x[i], y[i], i, radii[i],
                    &radii, &x, &y, max_radius,
                    bx, by, bw,
                    &mut cvx, &mut cvy,
                );
                vx[i] += cvx * collision_str;
                vy[i] += cvy * collision_str;
            }
            } // end collision tick
        }

        // Link (attraction) — pull connected nodes toward target distance
        for &(a, b) in &edge_pairs {
            let dx = x[b] - x[a];
            let dy = y[b] - y[a];
            let dist = (dx * dx + dy * dy).sqrt().max(0.1);
            let bias = degree[b] as f64 / (degree[a] + degree[b]).max(1) as f64;
            let force = (dist - link_distance) / dist * link_strength * alpha;
            let fx = dx * force;
            let fy = dy * force;
            vx[a] += fx * bias;
            vy[a] += fy * bias;
            vx[b] -= fx * (1.0 - bias);
            vy[b] -= fy * (1.0 - bias);
        }

        // Centering force — prevents drift
        let cx = x.iter().sum::<f64>() / n as f64;
        let cy = y.iter().sum::<f64>() / n as f64;
        for i in 0..n {
            x[i] -= cx;
            y[i] -= cy;
        }

        // Gravity — pull high-PageRank nodes toward center so hubs cluster together
        let gravity_strength = 0.03_f64;
        for i in 0..n {
            let pr_factor = nodes[i].page_rank / max_pr;
            let pull = gravity_strength * pr_factor * alpha;
            vx[i] -= x[i] * pull;
            vy[i] -= y[i] * pull;
        }

        // Velocity Verlet integration
        for i in 0..n {
            vx[i] *= 1.0 - velocity_decay;
            vy[i] *= 1.0 - velocity_decay;
            x[i] += vx[i];
            y[i] += vy[i];
        }
    }

    // Write positions (round for compact JSON)
    for i in 0..n {
        nodes[i].x = (x[i] * 10.0).round() / 10.0;
        nodes[i].y = (y[i] * 10.0).round() / 10.0;
    }
}

/// Generate topics graph data: nodes are tags, edges connect tags that co-occur on pages.
pub fn generate_topics_data(
    store: &PageStore,
    config: &SiteConfig,
    output_dir: &Path,
) -> Result<()> {
    use std::collections::HashMap;

    let public_pages = store.public_pages(&config.content);

    // Collect tag counts
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    // Collect co-occurrence edges: (tag_a, tag_b) → weight (number of shared pages)
    let mut cooccurrence: HashMap<(String, String), usize> = HashMap::new();

    for page in &public_pages {
        let tags: Vec<String> = page.meta.tags.iter().map(|t| t.to_lowercase()).collect();
        for tag in &tags {
            *tag_counts.entry(tag.clone()).or_default() += 1;
        }
        // All pairs of tags on this page create an edge
        for i in 0..tags.len() {
            for j in (i + 1)..tags.len() {
                let (a, b) = if tags[i] < tags[j] {
                    (tags[i].clone(), tags[j].clone())
                } else {
                    (tags[j].clone(), tags[i].clone())
                };
                *cooccurrence.entry((a, b)).or_default() += 1;
            }
        }
    }

    #[derive(Serialize)]
    struct TopicNode {
        id: String,
        name: String,
        count: usize,
        url: String,
    }

    #[derive(Serialize)]
    struct TopicEdge {
        source: String,
        target: String,
        weight: usize,
    }

    #[derive(Serialize)]
    struct TopicsData {
        nodes: Vec<TopicNode>,
        edges: Vec<TopicEdge>,
    }

    let nodes: Vec<TopicNode> = tag_counts
        .iter()
        .map(|(tag, count)| TopicNode {
            id: tag.clone(),
            name: tag.clone(),
            count: *count,
            url: format!("/tags/{}", slug::slugify(tag)),
        })
        .collect();

    let edges: Vec<TopicEdge> = cooccurrence
        .into_iter()
        .map(|((a, b), weight)| TopicEdge {
            source: a,
            target: b,
            weight,
        })
        .collect();

    let data = TopicsData { nodes, edges };
    let json = serde_json::to_string(&data)?;
    std::fs::write(output_dir.join("topics-data.json"), json)?;

    Ok(())
}

/// Generate per-page minimap data: local neighborhood graph within N hops.
#[derive(Serialize)]
pub struct MinimapData {
    pub nodes: Vec<MinimapNode>,
    pub edges: Vec<MinimapEdge>,
}

#[derive(Serialize)]
pub struct MinimapNode {
    pub id: String,
    pub title: String,
    pub url: String,
    pub current: bool,
}

#[derive(Serialize)]
pub struct MinimapEdge {
    pub source: String,
    pub target: String,
}

pub fn get_minimap_data(
    page_id: &str,
    store: &PageStore,
    depth: usize,
) -> MinimapData {
    let mut visited: HashSet<String> = HashSet::new();
    let mut frontier: Vec<String> = vec![page_id.to_string()];
    visited.insert(page_id.to_string());

    // BFS up to `depth` hops
    for _ in 0..depth {
        let mut next_frontier = Vec::new();
        for node_id in &frontier {
            // Forward links
            if let Some(targets) = store.forward_links.get(node_id) {
                for target in targets {
                    if visited.insert(target.clone()) {
                        next_frontier.push(target.clone());
                    }
                }
            }
            // Backlinks
            if let Some(sources) = store.backlinks.get(node_id) {
                for source in sources {
                    if visited.insert(source.clone()) {
                        next_frontier.push(source.clone());
                    }
                }
            }
        }
        frontier = next_frontier;
    }

    let nodes: Vec<MinimapNode> = visited
        .iter()
        .filter_map(|id| {
            store.pages.get(id).map(|page| MinimapNode {
                id: id.clone(),
                title: page.meta.title.clone(),
                url: format!("/{}", id),
                current: id == page_id,
            })
        })
        .collect();

    let node_ids: HashSet<&String> = visited.iter().collect();
    let mut edges = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();

    for id in &visited {
        if let Some(targets) = store.forward_links.get(id) {
            for target in targets {
                if node_ids.contains(target) {
                    let key = if id < target {
                        (id.clone(), target.clone())
                    } else {
                        (target.clone(), id.clone())
                    };
                    if seen.insert(key) {
                        edges.push(MinimapEdge {
                            source: id.clone(),
                            target: target.clone(),
                        });
                    }
                }
            }
        }
    }

    MinimapData { nodes, edges }
}
