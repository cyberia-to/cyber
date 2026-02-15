---
tags: cyber, cyberia
crystal-type: entity
crystal-domain: cyberia
---
- # Sensor Network
- a distributed system that transforms physical measurements into persistent, queryable knowledge
- ## architecture
  sensor networks bridge the physical and digital. data flows from measurement devices through content-addressed storage into a knowledge graph where it gains context and permanence
  
  the pipeline:
  ```
  physical world → sensor → measurement → IPFS → particle → cyberlink → knowledge graph
  ```
  
  each step:
  1. `measure`: [[sensor]] captures temperature, humidity, soil moisture, rainfall, light
  2. `hash`: measurement bundle → content-addressed file → [[IPFS]] CID
  3. `store`: CID becomes a [[particle]] in [[Bostrom]]
  4. `link`: [[neuron]] creates [[cyberlink]] from sensor particle to location, [[species]], time
  5. `rank`: [[rank]] algorithm surfaces most relevant environmental patterns
- ## sensor types → particle types
  | sensor | what it measures | links to |
  |--------|-----------------|----------|
  | soil moisture probe | water content at depth | [[species]] root zones, [[water]] system |
  | weather station | temp, humidity, rain, wind | climate patterns, [[ecosystem]] dynamics |
  | dendrometer | tree growth rate | [[species]] health, carbon sequestment |
  | camera trap | animal activity | [[species]] presence, behavior patterns |
  | pH meter | soil acidity | [[species]] suitability, amendment needs |
  | light sensor | canopy penetration | [[species]] shade tolerance mapping |
- ## why on-chain storage
  `permanence`: decade-long datasets compound in value. [[IPFS]] + [[Bostrom]] preserve observations across time
  
  `queryable`: "which [[species]] grows best at this soil moisture?" resolves through [[search]] against the [[knowledge graph]]
  
  `composable`: any agent can [[cyberlink]] sensor data to new analyses. observations become substrate for [[Superintelligence]]
  
  `verifiable`: readings carry timestamps, content hashes, and location links. tampering becomes evident through hash mismatch
- ## cyberia implementation
  cyberia deploys sensors across the estate: [[water]] monitoring, soil probes, weather stations, dendrometers. each measurement flows through the pipeline into the [[knowledge graph]]
  
  a cyberia sensor node:
  ```
  every 15 min:
      readings = collect_sensors()
      cid = ipfs_add(json(readings))
      cyberlink(sensor_cid, cid, "measurement")
      cyberlink(cid, location_cid, "measured_at")
      cyberlink(cid, species_cid, "relevant_to")   // if in species zone
  ```
  
  cost: one [[cyberlink]] transaction per reading. at 96 readings/day, the [[bandwidth]] cost is trivial for a [[neuron]] with staked [[CYB]]
- ## capabilities
  `relevance ranking`: environmental conditions rank by correlation with [[species]] performance
  
  `early warning`: anomaly detection across the sensor grid surfaces alerts through [[knowledge graph]] queries
  
  `emergent patterns`: the forest teaches the protocol what matters. the protocol remembers what the forest says
- ## existing networks
  [sensors.social](https://sensors.social/#/remote/pm10/3/-8.2332/115.1367)