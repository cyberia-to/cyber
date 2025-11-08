icon:: 🦠
alias:: cyber rank, particles weight, particles weights, cyberanks
- probability of [[particle]] observation by [[random walking]] [[neuron]]
- weighted on [[attention]] and [[will]] of [[neuron]]
- fundamental factor of [[implicit knowledge]]
- basically its plain old [[pagerank]] weighted by specific [[tokens]]
	- | feature                | [[pagerank]]                                     | cyberank                                                                 |
	  |------------------------|----------------------------------------------|--------------------------------------------------------------------------|
	  | input structure    | directed graph with edges indicating links   |  [[cybergraph]]          |
	  | damping factor     | typically set to 0.85                        | [[consensus parameter]]                  |
	  | link representation| edges with equal weight     | [[attention]] and [[will]] token                  |
	  | handling dangling nodes | distributed uniformly among all nodes      | adjusted rank calculation considering dangling nodes explicitly          |
	  | rank initialization| uniformly distributed initial ranks          | starts with all ranks initialized to zero                                |
	  | normalization     | ensures rank sum equals one                   | implicit normalization through rank adjustments and damping factor       |
- pseudocode in python
	- ```python
	  import functools
	  import operator
	  import collections
	  def cyberank(cyberlinks: list, tolerance: float = 0.001, damping_factor: float = 0.8):
	      cyberlinks_dict = dict(functools.reduce(operator.add, map(collections.Counter, cyberlinks)))
	      objects = list(set([item for t in [list(x.keys())[0] for x in cyberlinks] for item in t]))
	      rank = [0] * len(objects)
	      size = len(objects)
	      default_rank = (1.0 - damping_factor) / size
	      dangling_nodes = [obj for obj in objects if obj not in [list(cyberlink.keys())[0][1] for cyberlink in cyberlinks]]
	      dangling_nodes_size = len(dangling_nodes)
	      inner_product_over_size = default_rank * (dangling_nodes_size / size)
	      default_rank_with_correction = (damping_factor * inner_product_over_size) + default_rank
	      change = tolerance + 1
	      steps = 0
	      prevrank = [0] * len(objects)
	  while change > tolerance:
	      for obj in objects:
	          obj_index = objects.index(obj)
	          ksum = 0
	          income_cyberlinks = [income_cyberlink for income_cyberlink in [list(x.keys())[0] for x in cyberlinks] if income_cyberlink[1] == obj]
	          for cyberlink in income_cyberlinks:
	              linkStake = cyberlinks_dict[cyberlink]
	              outcome_cyberlinks = [outcome_cyberlink for outcome_cyberlink in [list(x.keys())[0] for x in cyberlinks] if outcome_cyberlink[0] == cyberlink[0]]
	              jCidOutStake = sum([cyberlinks_dict[outcome_cyberlink] for outcome_cyberlink in outcome_cyberlinks])
	              if linkStake == 0 or jCidOutStake == 0:
	                  continue
	              weight = linkStake / jCidOutStake
	              ksum = prevrank[obj_index] * weight + ksum
	          rank[obj_index] = ksum * damping_factor + default_rank_with_correction
	      change = abs(max(rank) - max(prevrank))
	      prevrank = rank
	      steps += 1
	  res = list(zip(objects, rank))
	  res.sort(reverse=True, key=lambda x: x[1])
	  return res
	  ```
- [[go-cyber]] [implementation of cyberank in go](https://github.com/cybercongress/go-cyber/blob/main/x/rank/keeper/calculate_gpu.go#L22)
- cyberank is being used as factor in
	- [[standard inference]]
	- [[syntropy]]
	- [[karma]]
	- local sorting in [[cyb]]
- display cyberank in apps
	- 🦠 emoji icon virus
	- examples
		- full number: 176 711 938 🦠
		- game number: 176 🦠🦠🦠