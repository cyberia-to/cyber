// Topics graph visualization — full-screen force-directed cloud
(function() {
  function initTopicsGraph(containerId, opts) {
    const container = document.getElementById(containerId);
    if (!container) return;

    opts = opts || {};
    const compact = opts.compact || false;
    const maxNodes = opts.maxNodes || 0;

    function loadD3(cb) {
      if (window.d3) return cb();
      const s = document.createElement('script');
      s.src = 'https://d3js.org/d3.v7.min.js';
      s.onload = cb;
      document.head.appendChild(s);
    }

    loadD3(function() {
      fetch('/topics-data.json')
        .then(r => r.json())
        .then(data => render(data))
        .catch(err => {
          container.innerHTML = '<p style="opacity:0.5">No topics data.</p>';
          console.error(err);
        });
    });

    function render(data) {
      data.nodes.sort((a, b) => b.count - a.count);

      if (maxNodes > 0 && data.nodes.length > maxNodes) {
        const keepIds = new Set(data.nodes.slice(0, maxNodes).map(n => n.id));
        data.nodes = data.nodes.filter(n => keepIds.has(n.id));
        data.edges = data.edges.filter(e => keepIds.has(e.source) && keepIds.has(e.target));
      }

      const w = container.clientWidth || window.innerWidth;
      const h = container.clientHeight || window.innerHeight;

      const maxCount = d3.max(data.nodes, d => d.count) || 1;
      const radiusScale = d3.scaleSqrt().domain([1, maxCount]).range(compact ? [6, 28] : [8, 45]);
      const fontScale = d3.scaleSqrt().domain([1, maxCount]).range(compact ? [9, 16] : [11, 24]);

      const svg = d3.select(container)
        .append('svg')
        .attr('width', '100%')
        .attr('height', '100%')
        .attr('viewBox', [0, 0, w, h]);

      // Glow filter
      const defs = svg.append('defs');
      const filter = defs.append('filter').attr('id', 'glow-' + containerId);
      filter.append('feGaussianBlur').attr('stdDeviation', '3').attr('result', 'blur');
      filter.append('feComposite').attr('in', 'SourceGraphic').attr('in2', 'blur').attr('operator', 'over');

      const g = svg.append('g');
      svg.call(d3.zoom()
        .scaleExtent([0.2, 6])
        .on('zoom', (event) => g.attr('transform', event.transform)));

      // Build adjacency for highlight
      const adjacency = {};
      data.nodes.forEach(n => adjacency[n.id] = new Set());
      data.edges.forEach(e => {
        adjacency[e.source] = adjacency[e.source] || new Set();
        adjacency[e.target] = adjacency[e.target] || new Set();
        adjacency[e.source].add(e.target);
        adjacency[e.target].add(e.source);
      });

      const simulation = d3.forceSimulation(data.nodes)
        .force('link', d3.forceLink(data.edges)
          .id(d => d.id)
          .distance(d => compact ? 60 : 100)
          .strength(d => Math.min(0.6, d.weight * 0.12)))
        .force('charge', d3.forceManyBody()
          .strength(compact ? -50 : -120)
          .distanceMax(compact ? 300 : 500))
        .force('center', d3.forceCenter(w / 2, h / 2).strength(0.05))
        .force('x', d3.forceX(w / 2).strength(0.03))
        .force('y', d3.forceY(h / 2).strength(0.03))
        .force('collision', d3.forceCollide()
          .radius(d => radiusScale(d.count) + (compact ? 8 : 14))
          .strength(0.7));

      // Edges
      const link = g.append('g')
        .selectAll('line')
        .data(data.edges)
        .join('line')
        .attr('stroke', 'var(--color-primary)')
        .attr('stroke-opacity', d => Math.min(0.35, 0.05 + d.weight * 0.06))
        .attr('stroke-width', d => Math.min(2.5, 0.5 + d.weight * 0.4));

      // Node groups
      const nodeGroup = g.append('g')
        .selectAll('g')
        .data(data.nodes)
        .join('g')
        .style('cursor', 'pointer')
        .call(drag(simulation));

      // Outer glow circle
      nodeGroup.append('circle')
        .attr('r', d => radiusScale(d.count) + 4)
        .attr('fill', 'var(--color-primary)')
        .attr('fill-opacity', 0)
        .attr('class', 'node-glow');

      // Main circle
      nodeGroup.append('circle')
        .attr('r', d => radiusScale(d.count))
        .attr('fill', 'var(--color-primary)')
        .attr('fill-opacity', d => 0.06 + (d.count / maxCount) * 0.14)
        .attr('stroke', 'var(--color-primary)')
        .attr('stroke-width', d => 1 + (d.count / maxCount) * 1.5)
        .attr('stroke-opacity', d => 0.3 + (d.count / maxCount) * 0.5)
        .attr('class', 'node-circle');

      // Labels
      nodeGroup.append('text')
        .text(d => {
          const max = compact ? 14 : 22;
          return d.name.length > max ? d.name.slice(0, max - 1) + '\u2026' : d.name;
        })
        .attr('font-size', d => fontScale(d.count) + 'px')
        .attr('fill', 'var(--color-text)')
        .attr('text-anchor', 'middle')
        .attr('dominant-baseline', 'central')
        .attr('font-weight', d => d.count > maxCount * 0.2 ? '600' : '400')
        .attr('opacity', d => 0.5 + (d.count / maxCount) * 0.5)
        .style('pointer-events', 'none')
        .attr('class', 'node-label');

      // Count badge for big nodes
      if (!compact) {
        nodeGroup.filter(d => d.count > maxCount * 0.1)
          .append('text')
          .text(d => d.count)
          .attr('font-size', '9px')
          .attr('fill', 'var(--color-primary)')
          .attr('text-anchor', 'middle')
          .attr('dy', d => radiusScale(d.count) + 12)
          .attr('opacity', 0.4)
          .style('pointer-events', 'none');
      }

      // Tooltip
      const tooltip = d3.select(container)
        .append('div')
        .style('position', 'absolute')
        .style('display', 'none')
        .style('background', 'var(--color-surface)')
        .style('border', '1px solid var(--color-primary)')
        .style('padding', '8px 14px')
        .style('border-radius', '8px')
        .style('font-size', '13px')
        .style('pointer-events', 'none')
        .style('box-shadow', '0 0 20px rgba(34,197,94,0.15)')
        .style('z-index', '10')
        .style('backdrop-filter', 'blur(8px)');

      // Interactions
      nodeGroup.on('mouseover', function(event, d) {
        tooltip.style('display', 'block')
          .html('<strong style="font-size:15px">' + d.name + '</strong><br><span style="font-size:12px;opacity:0.6">' + d.count + ' pages</span>');

        // Highlight this node
        d3.select(this).select('.node-circle')
          .attr('fill-opacity', 0.3)
          .attr('stroke-opacity', 1)
          .attr('stroke-width', 3);
        d3.select(this).select('.node-glow')
          .attr('fill-opacity', 0.08)
          .attr('filter', 'url(#glow-' + containerId + ')');
        d3.select(this).select('.node-label')
          .attr('opacity', 1)
          .attr('font-weight', '700');

        // Dim non-connected, highlight connected
        const connected = adjacency[d.id] || new Set();
        nodeGroup.each(function(n) {
          if (n.id !== d.id && !connected.has(n.id)) {
            d3.select(this).attr('opacity', 0.15);
          }
        });
        link.attr('stroke-opacity', e => {
          const src = typeof e.source === 'object' ? e.source.id : e.source;
          const tgt = typeof e.target === 'object' ? e.target.id : e.target;
          return (src === d.id || tgt === d.id) ? 0.7 : 0.03;
        }).attr('stroke-width', e => {
          const src = typeof e.source === 'object' ? e.source.id : e.source;
          const tgt = typeof e.target === 'object' ? e.target.id : e.target;
          return (src === d.id || tgt === d.id) ? 3 : Math.min(2.5, 0.5 + e.weight * 0.4);
        });
      }).on('mousemove', function(event) {
        const rect = container.getBoundingClientRect();
        tooltip.style('left', (event.clientX - rect.left + 15) + 'px')
          .style('top', (event.clientY - rect.top - 10) + 'px');
      }).on('mouseout', function() {
        tooltip.style('display', 'none');
        nodeGroup.attr('opacity', 1);
        nodeGroup.each(function(n) {
          d3.select(this).select('.node-circle')
            .attr('fill-opacity', 0.06 + (n.count / maxCount) * 0.14)
            .attr('stroke-opacity', 0.3 + (n.count / maxCount) * 0.5)
            .attr('stroke-width', 1 + (n.count / maxCount) * 1.5);
          d3.select(this).select('.node-glow').attr('fill-opacity', 0).attr('filter', null);
          d3.select(this).select('.node-label')
            .attr('opacity', 0.5 + (n.count / maxCount) * 0.5)
            .attr('font-weight', n.count > maxCount * 0.2 ? '600' : '400');
        });
        link.attr('stroke-opacity', e => Math.min(0.35, 0.05 + e.weight * 0.06))
          .attr('stroke-width', e => Math.min(2.5, 0.5 + e.weight * 0.4));
      }).on('click', function(event, d) {
        window.location.href = d.url;
      });

      simulation.on('tick', () => {
        link
          .attr('x1', d => d.source.x)
          .attr('y1', d => d.source.y)
          .attr('x2', d => d.target.x)
          .attr('y2', d => d.target.y);
        nodeGroup.attr('transform', d => 'translate(' + d.x + ',' + d.y + ')');
      });

      // Stats pill
      if (!compact) {
        var stats = document.createElement('div');
        stats.className = 'graph-stats';
        stats.innerHTML = data.nodes.length + ' topics &middot; ' + data.edges.length + ' connections';
        container.appendChild(stats);
      }
    }

    function drag(simulation) {
      return d3.drag()
        .on('start', (event) => {
          if (!event.active) simulation.alphaTarget(0.3).restart();
          event.subject.fx = event.subject.x;
          event.subject.fy = event.subject.y;
        })
        .on('drag', (event) => {
          event.subject.fx = event.x;
          event.subject.fy = event.y;
        })
        .on('end', (event) => {
          if (!event.active) simulation.alphaTarget(0);
          event.subject.fx = null;
          event.subject.fy = null;
        });
    }
  }

  window.initTopicsGraph = initTopicsGraph;
})();
