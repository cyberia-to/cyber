// Graph visualization — Canvas-based, instant render for large graphs
(function() {
  const container = document.getElementById('graph-container');
  if (!container) return;

  if (window.d3) {
    initGraph();
  } else {
    const script = document.createElement('script');
    script.src = 'https://d3js.org/d3.v7.min.js';
    script.onload = () => initGraph();
    script.onerror = () => {
      container.innerHTML = '<p style="color:var(--color-text);opacity:0.5;text-align:center;padding-top:40vh">Could not load D3.js — check network connection</p>';
    };
    document.head.appendChild(script);
  }

  function initGraph() {
    if (window.__GRAPH_DATA) {
      try { renderGraph(window.__GRAPH_DATA); }
      catch (err) {
        container.innerHTML = '<p style="color:var(--color-text);opacity:0.5;text-align:center;padding-top:40vh">Graph error: ' + err.message + '</p>';
        console.error(err);
      }
    } else {
      fetch('/graph-data.json')
        .then(r => { if (!r.ok) throw new Error(r.status + ' ' + r.statusText); return r.json(); })
        .then(data => renderGraph(data))
        .catch(err => {
          container.innerHTML = '<p style="color:var(--color-text);opacity:0.5;text-align:center;padding-top:40vh">Failed to load graph data: ' + err.message + '</p>';
          console.error(err);
        });
    }
  }

  function renderGraph(data) {
    const dpr = window.devicePixelRatio || 1;
    const w = container.clientWidth || window.innerWidth;
    const h = container.clientHeight || window.innerHeight;

    // Canvas setup
    const canvas = document.createElement('canvas');
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    canvas.style.width = w + 'px';
    canvas.style.height = h + 'px';
    container.appendChild(canvas);
    const ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);

    // Read CSS colors
    const style = getComputedStyle(document.documentElement);
    const colorPrimary = style.getPropertyValue('--color-primary').trim() || '#22c55e';
    const colorText = style.getPropertyValue('--color-text').trim() || '#e5e5e5';
    const colorBg = style.getPropertyValue('--color-bg').trim() || '#0a0a0a';

    // Pre-compute adjacency
    const adj = {};
    data.nodes.forEach(n => { adj[n.id] = new Set(); });
    data.edges.forEach(e => {
      adj[e.source] = adj[e.source] || new Set();
      adj[e.target] = adj[e.target] || new Set();
      adj[e.source].add(e.target);
      adj[e.target].add(e.source);
    });

    const maxFocus = d3.max(data.nodes, d => d.focus) || 0.001;
    const radiusScale = d3.scalePow().exponent(0.4).domain([0, maxFocus]).range([1.5, 20]);
    const opacityScale = d3.scalePow().exponent(0.4).domain([0, maxFocus]).range([0.08, 0.85]);

    // Top labeled nodes — by focus (identifies true hubs)
    const sorted = [...data.nodes].sort((a, b) => (b.focus || 0) - (a.focus || 0));
    const labelCount = Math.min(12, Math.max(3, Math.floor(data.nodes.length * 0.002)));
    const labelSet = new Set(sorted.slice(0, labelCount).map(n => n.id));

    // Node index for fast lookup
    const nodeById = {};
    data.nodes.forEach(n => { nodeById[n.id] = n; });

    // Transform state (must be before finishSetup is called)
    let transform = d3.zoomIdentity;
    let hoveredNode = null;
    let edgeRefs = [];

    // Stats element (created early so filter can update it)
    const stats = document.createElement('div');
    stats.className = 'graph-stats';

    // Tag filter state
    let activeTags = new Set();
    let visibleNodes = null; // null = show all, Set = filtered

    // Extract tags sorted by frequency, min 3 occurrences, max 20 pills
    const tagCounts = {};
    data.nodes.forEach(n => {
      (n.tags || []).forEach(t => { tagCounts[t] = (tagCounts[t] || 0) + 1; });
    });
    const topTags = Object.entries(tagCounts)
      .filter(([, count]) => count >= 3)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 20)
      .map(([tag]) => tag);

    // Restore filter from URL params
    const urlParams = new URLSearchParams(window.location.search);
    const urlTags = urlParams.get('tags');
    if (urlTags) {
      urlTags.split(',').forEach(t => {
        const trimmed = t.trim();
        if (trimmed && tagCounts[trimmed]) activeTags.add(trimmed);
      });
      updateVisibleNodes();
    }

    function syncFilterToURL() {
      const url = new URL(window.location);
      if (activeTags.size > 0) {
        url.searchParams.set('tags', Array.from(activeTags).join(','));
      } else {
        url.searchParams.delete('tags');
      }
      history.replaceState(null, '', url);
    }

    // Persist viewport in sessionStorage
    const VIEWPORT_KEY = 'graph-viewport';
    function saveViewport() {
      try {
        sessionStorage.setItem(VIEWPORT_KEY, JSON.stringify({
          x: transform.x, y: transform.y, k: transform.k
        }));
      } catch (e) {}
    }
    function loadViewport() {
      try {
        const raw = sessionStorage.getItem(VIEWPORT_KEY);
        if (raw) return JSON.parse(raw);
      } catch (e) {}
      return null;
    }

    function updateVisibleNodes() {
      if (activeTags.size === 0) {
        visibleNodes = null;
      } else {
        visibleNodes = new Set();
        data.nodes.forEach(n => {
          if ((n.tags || []).some(t => activeTags.has(t))) {
            visibleNodes.add(n.id);
          }
        });
      }
    }

    function buildFilterUI() {
      if (topTags.length === 0) return;
      const bar = document.createElement('div');
      bar.className = 'graph-filter';

      const allPill = document.createElement('button');
      allPill.className = 'graph-filter-pill active';
      allPill.textContent = 'all';
      allPill.addEventListener('click', () => {
        activeTags.clear();
        updateVisibleNodes();
        updatePillStates();
        updateStats();
        syncFilterToURL();
        draw();
      });
      bar.appendChild(allPill);

      topTags.forEach(tag => {
        const pill = document.createElement('button');
        pill.className = 'graph-filter-pill';
        pill.textContent = tag;
        pill.dataset.tag = tag;
        pill.addEventListener('click', () => {
          if (activeTags.has(tag)) {
            activeTags.delete(tag);
          } else {
            activeTags.add(tag);
          }
          updateVisibleNodes();
          updatePillStates();
          updateStats();
          syncFilterToURL();
          draw();
        });
        bar.appendChild(pill);
      });

      document.body.appendChild(bar);
    }

    function updatePillStates() {
      const pills = document.querySelectorAll('.graph-filter-pill');
      pills.forEach(pill => {
        const tag = pill.dataset.tag;
        if (!tag) {
          // "all" pill
          pill.classList.toggle('active', activeTags.size === 0);
        } else {
          pill.classList.toggle('active', activeTags.has(tag));
        }
      });
    }

    function updateStats() {
      if (!stats) return;
      if (visibleNodes) {
        const visEdges = edgeRefs.filter(e => {
          const sid = e.source.id || e.source;
          const tid = e.target.id || e.target;
          return visibleNodes.has(sid) || visibleNodes.has(tid);
        }).length;
        stats.innerHTML = visibleNodes.size + ' / ' + data.nodes.length + ' pages &middot; ' + visEdges + ' connections';
      } else {
        stats.innerHTML = data.nodes.length + ' pages &middot; ' + data.edges.length + ' connections';
      }
    }

    // Check if positions are pre-computed (from build)
    const hasLayout = data.nodes.length > 0 && data.nodes.some(n => n.x !== 0 || n.y !== 0);

    if (hasLayout) {
      // Use percentile bounds (ignore outliers for better framing)
      const xs = data.nodes.map(d => d.x).sort((a, b) => a - b);
      const ys = data.nodes.map(d => d.y).sort((a, b) => a - b);
      const p = Math.max(1, Math.floor(data.nodes.length * 0.02));
      const minX = xs[p], maxX = xs[xs.length - 1 - p];
      const minY = ys[p], maxY = ys[ys.length - 1 - p];
      const graphW = maxX - minX || 1;
      const graphH = maxY - minY || 1;
      const scale = Math.min(w / graphW, h / graphH) * 0.85;
      const cx = (minX + maxX) / 2;
      const cy = (minY + maxY) / 2;
      data.nodes.forEach(d => {
        d.x = w / 2 + (d.x - cx) * scale;
        d.y = h / 2 + (d.y - cy) * scale;
      });
      finishSetup();
    } else {
      // Fallback: compute layout in browser
      data.nodes.forEach(d => {
        const angle = Math.random() * Math.PI * 2;
        const r = Math.random() * Math.min(w, h) * 0.35;
        d.x = w / 2 + Math.cos(angle) * r;
        d.y = h / 2 + Math.sin(angle) * r;
      });

      const overlay = document.createElement('div');
      overlay.style.cssText = 'position:absolute;top:0;left:0;width:100%;height:100%;display:flex;flex-direction:column;align-items:center;justify-content:center;z-index:20;pointer-events:none';
      const progText = document.createElement('div');
      progText.style.cssText = 'color:var(--color-primary);font-size:14px;font-weight:500;letter-spacing:0.05em';
      progText.textContent = 'Computing layout\u2026';
      const progBar = document.createElement('div');
      progBar.style.cssText = 'width:200px;height:2px;background:color-mix(in srgb,var(--color-primary) 15%,transparent);border-radius:1px;margin-top:12px;overflow:hidden';
      const progFill = document.createElement('div');
      progFill.style.cssText = 'width:0%;height:100%;background:var(--color-primary);border-radius:1px;transition:width 0.1s';
      progBar.appendChild(progFill);
      overlay.appendChild(progText);
      overlay.appendChild(progBar);
      container.appendChild(overlay);

      const simulation = d3.forceSimulation(data.nodes)
        .force('link', d3.forceLink(data.edges).id(d => d.id).distance(20).strength(0.3))
        .force('charge', d3.forceManyBody().strength(-15).distanceMax(250).theta(0.9))
        .force('center', d3.forceCenter(w / 2, h / 2))
        .force('collision', d3.forceCollide().radius(d => radiusScale(d.focus) + 2).strength(0.8).iterations(1))
        .alphaDecay(0.04)
        .stop();

      const maxTicks = 300;
      const chunkSize = 15;
      let ticksDone = 0;
      const t0 = performance.now();

      function tickChunk() {
        for (let i = 0; i < chunkSize && ticksDone < maxTicks; i++, ticksDone++) {
          simulation.tick();
          if (simulation.alpha() < 0.001) { ticksDone = maxTicks; break; }
        }
        const pct = Math.min(100, Math.round(ticksDone / maxTicks * 100));
        progFill.style.width = pct + '%';
        progText.textContent = 'Computing layout\u2026 ' + pct + '%';

        if (ticksDone < maxTicks) {
          setTimeout(tickChunk, 0);
        } else {
          overlay.remove();
          finishSetup();
        }
      }
      tickChunk();
    }

    function finishSetup() {
      // Build edge source/target refs (d3 replaces IDs with objects after simulation)
      edgeRefs = data.edges.map(e => ({
        source: typeof e.source === 'object' ? e.source : nodeById[e.source],
        target: typeof e.target === 'object' ? e.target : nodeById[e.target]
      }));

      buildFilterUI();
      updatePillStates();
      updateStats();
      draw();

      // Enable zoom + interactions
      const zoom = d3.zoom()
        .scaleExtent([0.1, 20])
        .on('zoom', (event) => {
          transform = event.transform;
          saveViewport();
          draw();
        });
      d3.select(canvas).call(zoom);

      // Restore saved viewport
      const saved = loadViewport();
      if (saved) {
        transform = d3.zoomIdentity.translate(saved.x, saved.y).scale(saved.k);
        d3.select(canvas).call(zoom.transform, transform);
      }

      setupInteractions();
    }

    function draw() {
      ctx.save();
      ctx.clearRect(0, 0, w, h);
      ctx.translate(transform.x, transform.y);
      ctx.scale(transform.k, transform.k);

      const k = transform.k;
      const connectedSet = hoveredNode ? (adj[hoveredNode.id] || new Set()) : null;

      // Determine if a node is visible (passes tag filter)
      const isVisible = visibleNodes ? (id) => visibleNodes.has(id) : () => true;

      // Draw edges — when hovering, or when filter active (show connections between visible nodes)
      if (hoveredNode || visibleNodes) {
        ctx.strokeStyle = colorPrimary;
        ctx.lineWidth = 0.8 / k;
        ctx.beginPath();
        for (const e of edgeRefs) {
          const sid = e.source.id || e.source;
          const tid = e.target.id || e.target;
          let show = false;
          if (hoveredNode && (sid === hoveredNode.id || tid === hoveredNode.id)) {
            show = true;
          } else if (visibleNodes && isVisible(sid) && isVisible(tid)) {
            show = true;
          }
          if (show) {
            ctx.moveTo(e.source.x, e.source.y);
            ctx.lineTo(e.target.x, e.target.y);
          }
        }
        ctx.globalAlpha = visibleNodes ? 0.15 : 0.3;
        ctx.stroke();
        ctx.globalAlpha = 1;
      }

      // Draw nodes
      for (const n of data.nodes) {
        const r = radiusScale(n.focus);
        const nodeVisible = isVisible(n.id);
        let alpha = opacityScale(n.focus);

        // Tag filter fading
        if (visibleNodes && !nodeVisible) {
          alpha = 0.02;
        }

        if (hoveredNode) {
          if (n.id === hoveredNode.id) {
            alpha = 1;
          } else if (connectedSet.has(n.id)) {
            alpha = visibleNodes && !nodeVisible ? 0.15 : 0.7;
          } else {
            alpha = 0.03;
          }
        }

        ctx.beginPath();
        ctx.arc(n.x, n.y, r, 0, Math.PI * 2);
        ctx.fillStyle = colorPrimary;
        ctx.globalAlpha = alpha;
        ctx.fill();

        // Glow on hovered node
        if (hoveredNode && n.id === hoveredNode.id) {
          ctx.shadowColor = colorPrimary;
          ctx.shadowBlur = 12;
          ctx.strokeStyle = colorPrimary;
          ctx.lineWidth = 1.5 / k;
          ctx.stroke();
          ctx.shadowBlur = 0;
        }

        ctx.globalAlpha = 1;
      }

      // Draw labels — above nodes, clean
      const showLabelsAtZoom = k >= 0.5;
      if (showLabelsAtZoom) {
        ctx.textAlign = 'center';
        ctx.textBaseline = 'bottom';

        for (const n of data.nodes) {
          const r = radiusScale(n.focus);
          let show = false;
          let alpha = 0.7;

          const nodeVisible = isVisible(n.id);

          if (hoveredNode) {
            if (n.id === hoveredNode.id || connectedSet.has(n.id)) {
              show = true;
              alpha = n.id === hoveredNode.id ? 1 : 0.75;
            } else if (labelSet.has(n.id)) {
              show = true;
              alpha = 0.15;
            }
          } else if (visibleNodes) {
            // Filter active: label all visible nodes
            if (nodeVisible) {
              show = true;
              alpha = 0.8;
            }
          } else {
            // Default: show top labels, more at higher zoom
            if (labelSet.has(n.id)) {
              show = true;
              alpha = 0.7;
            } else if (k >= 2 && n.focus > maxFocus * 0.08) {
              show = true;
              alpha = 0.5;
            } else if (k >= 4 && n.focus > maxFocus * 0.02) {
              show = true;
              alpha = 0.4;
            } else if (k >= 6) {
              show = true;
              alpha = 0.35;
            }
          }

          if (!show) continue;

          const fontSize = Math.max(9, Math.min(14, 10 + r * 0.2)) / k;
          ctx.font = '500 ' + fontSize + 'px system-ui, sans-serif';
          ctx.globalAlpha = alpha;

          // Text shadow for readability
          ctx.strokeStyle = colorBg;
          ctx.lineWidth = 3 / k;
          ctx.lineJoin = 'round';
          ctx.strokeText(n.title, n.x, n.y - r - 3 / k);

          ctx.fillStyle = colorText;
          ctx.fillText(n.title, n.x, n.y - r - 3 / k);
        }
      }

      ctx.globalAlpha = 1;
      ctx.restore();
    }

    // Hit detection — prefer visible nodes when filter active
    function findNode(mx, my) {
      const gx = (mx - transform.x) / transform.k;
      const gy = (my - transform.y) / transform.k;
      let closest = null;
      let closestDist = Infinity;
      for (const n of data.nodes) {
        if (visibleNodes && !visibleNodes.has(n.id)) continue;
        const r = radiusScale(n.focus);
        const hitR = Math.max(r, 5 / transform.k);
        const dx = gx - n.x;
        const dy = gy - n.y;
        const dist = dx * dx + dy * dy;
        if (dist < hitR * hitR && dist < closestDist) {
          closest = n;
          closestDist = dist;
        }
      }
      return closest;
    }

    // Tooltip
    const tooltip = document.createElement('div');
    tooltip.style.cssText = 'position:absolute;display:none;background:color-mix(in srgb,var(--color-bg) 85%,transparent);backdrop-filter:blur(8px);-webkit-backdrop-filter:blur(8px);border:1px solid var(--color-primary);padding:8px 14px;border-radius:8px;font-size:13px;pointer-events:none;box-shadow:0 0 20px rgba(34,197,94,0.15);z-index:10';
    container.appendChild(tooltip);

    let dragNode = null;
    let isDragging = false;

    function setupInteractions() {
      canvas.addEventListener('mousemove', (event) => {
        if (dragNode) {
          isDragging = true;
          const rect = canvas.getBoundingClientRect();
          dragNode.x = (event.clientX - rect.left - transform.x) / transform.k;
          dragNode.y = (event.clientY - rect.top - transform.y) / transform.k;
          draw();
          return;
        }
        const rect = canvas.getBoundingClientRect();
        const mx = event.clientX - rect.left;
        const my = event.clientY - rect.top;
        const node = findNode(mx, my);

        if (node !== hoveredNode) {
          hoveredNode = node;
          draw();
        }

        if (node) {
          canvas.style.cursor = 'pointer';
          tooltip.style.display = 'block';
          tooltip.innerHTML = '<strong style="font-size:15px">' + node.title + '</strong><br><span style="font-size:12px;opacity:0.6">π ' + ((node.focus || 0) * 100).toFixed(2) + '% · ' + node.linkCount + ' links</span>';
          tooltip.style.left = (event.clientX - rect.left + 15) + 'px';
          tooltip.style.top = (event.clientY - rect.top - 10) + 'px';
        } else {
          canvas.style.cursor = 'grab';
          tooltip.style.display = 'none';
        }
      });

      canvas.addEventListener('mouseleave', () => {
        hoveredNode = null;
        tooltip.style.display = 'none';
        draw();
      });

      canvas.addEventListener('click', (event) => {
        if (isDragging) return;
        const rect = canvas.getBoundingClientRect();
        const node = findNode(event.clientX - rect.left, event.clientY - rect.top);
        if (node) window.location.href = node.url;
      });

      canvas.addEventListener('mousedown', (event) => {
        const rect = canvas.getBoundingClientRect();
        const node = findNode(event.clientX - rect.left, event.clientY - rect.top);
        if (node) {
          dragNode = node;
          isDragging = false;
        }
      });

      canvas.addEventListener('mouseup', () => {
        dragNode = null;
        isDragging = false;
      });

      window.addEventListener('resize', () => {
        const nw = container.clientWidth || window.innerWidth;
        const nh = container.clientHeight || window.innerHeight;
        canvas.width = nw * dpr;
        canvas.height = nh * dpr;
        canvas.style.width = nw + 'px';
        canvas.style.height = nh + 'px';
        ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
        draw();
      });
    }

    // Stats (appended early so filter UI can update it)
    stats.innerHTML = data.nodes.length + ' pages &middot; ' + data.edges.length + ' connections';
    container.appendChild(stats);
  }
})();

// Minimap widget for per-page graph
(function() {
  const container = document.getElementById('minimap-container');
  if (!container) return;

  const pageId = container.dataset.pageId;
  const depth = parseInt(container.dataset.depth) || 2;
  if (!pageId) return;

  function loadAndRender() {
    fetch('/graph-data.json')
      .then(r => r.json())
      .then(fullGraph => {
        const data = extractNeighborhood(fullGraph, pageId, depth);
        if (data.nodes.length > 1) {
          renderMinimap(data);
        } else {
          container.innerHTML = '<p style="color:#888;font-size:12px">No connections</p>';
        }
      })
      .catch(() => {});
  }

  function extractNeighborhood(graph, centerId, maxDepth) {
    const adj = {};
    const nodeMap = {};
    graph.nodes.forEach(n => { nodeMap[n.id] = n; adj[n.id] = []; });
    graph.edges.forEach(e => {
      const s = typeof e.source === 'object' ? e.source.id : e.source;
      const t = typeof e.target === 'object' ? e.target.id : e.target;
      if (adj[s]) adj[s].push(t);
      if (adj[t]) adj[t].push(s);
    });

    const visited = new Set([centerId]);
    let frontier = [centerId];
    for (let d = 0; d < maxDepth; d++) {
      const next = [];
      for (const nid of frontier) {
        for (const neighbor of (adj[nid] || [])) {
          if (!visited.has(neighbor)) {
            visited.add(neighbor);
            next.push(neighbor);
          }
        }
      }
      frontier = next;
    }

    const nodes = Array.from(visited).filter(id => nodeMap[id]).map(id => ({
      id, title: nodeMap[id].title, url: nodeMap[id].url, current: id === centerId
    }));
    const nodeSet = new Set(visited);
    const edges = graph.edges.filter(e => {
      const s = typeof e.source === 'object' ? e.source.id : e.source;
      const t = typeof e.target === 'object' ? e.target.id : e.target;
      return nodeSet.has(s) && nodeSet.has(t);
    }).map(e => ({
      source: typeof e.source === 'object' ? e.source.id : e.source,
      target: typeof e.target === 'object' ? e.target.id : e.target
    }));
    return { nodes, edges };
  }

  if (window.d3) {
    loadAndRender();
  } else {
    const script = document.createElement('script');
    script.src = 'https://d3js.org/d3.v7.min.js';
    script.onload = loadAndRender;
    document.head.appendChild(script);
  }

  function renderMinimap(data) {
    const w = container.clientWidth || 280;
    const h = 200;

    const svg = d3.select(container)
      .append('svg')
      .attr('width', w)
      .attr('height', h);

    const g = svg.append('g');

    const simulation = d3.forceSimulation(data.nodes)
      .force('link', d3.forceLink(data.edges).id(d => d.id).distance(40).strength(0.5))
      .force('charge', d3.forceManyBody().strength(-60))
      .force('center', d3.forceCenter(w / 2, h / 2))
      .force('collision', d3.forceCollide(8));

    const link = g.append('g')
      .selectAll('line')
      .data(data.edges)
      .join('line')
      .attr('stroke', 'var(--color-border)')
      .attr('stroke-opacity', 0.5);

    const node = g.append('g')
      .selectAll('circle')
      .data(data.nodes)
      .join('circle')
      .attr('r', d => d.current ? 6 : 4)
      .attr('fill', d => d.current ? 'var(--color-primary)' : 'var(--color-secondary)')
      .attr('fill-opacity', d => d.current ? 1 : 0.6)
      .style('cursor', 'pointer')
      .on('click', (event, d) => { window.location.href = d.url; });

    const label = g.append('g')
      .selectAll('text')
      .data(data.nodes)
      .join('text')
      .text(d => d.title.length > 15 ? d.title.slice(0, 13) + '\u2026' : d.title)
      .attr('font-size', '8px')
      .attr('fill', 'var(--color-text)')
      .attr('text-anchor', 'middle')
      .attr('dy', d => d.current ? -10 : -8)
      .style('pointer-events', 'none');

    simulation.on('tick', () => {
      link
        .attr('x1', d => d.source.x)
        .attr('y1', d => d.source.y)
        .attr('x2', d => d.target.x)
        .attr('y2', d => d.target.y);
      node
        .attr('cx', d => d.x)
        .attr('cy', d => d.y);
      label
        .attr('x', d => d.x)
        .attr('y', d => d.y);
    });
  }
})();
