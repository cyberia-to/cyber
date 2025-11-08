  - e = 6e6
  - memory per iter ~ o(7e6) entries
  - compute per iter ~ o(7e6) ops-equivalent
  - total ~ o(7e6 · k_ε)  

• v = 1e8, c = 12 (semi-sparse)
  - e = 1.2e9
  - memory per iter ~ o(1.2e9)
  - compute per iter ~ o(1.2e9)
  - total ~ o(1.2e9 · k_ε)  

• v = 1e10, c = v^{0.25} ≈ 100 (densifying)
  - e ≈ 1e12
  - memory/compute per iter ~ o(1e12)
  - total ~ o(1e12 · k_ε)  

takeaways
1) strict o(v) per-iteration is only realistic when connectivity is bounded (c = o(1)) and layouts are sparse  
2) total runtime is dominated by (v + e) times the mixing factor log(1/ε)/λ  
3) you can trade density for mixing: increase λ via teleportation, hierarchy, or degree caps instead of raw edges  
4) localized and monte carlo variants let you control constants and memory by limiting horizon or samples  
5) in decentralized deployments, communication and partitioning dominate — design for minimal edge cuts and steady streaming  

appendix: mapping to transformer intuition
• focus flow with capped c and h resembles sliding-window + sparse global tokens  
• teleportation ≈ global tokens or cls-like anchors improving mixing without quadratic blowup  
• hierarchy ≈ multi-scale attention layers that refine coarse focus  

open questions
• optimal sparsification policy that maximizes λ per edge budget  
• error bounds for localized focus vs global stationary distribution under degree caps  
• best-in-class partitioners and pipelines for billion-edge cybergraphs on commodity clusters