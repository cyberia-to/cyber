---
tags: cyber valley, cyberia, robotics, energy, species, research
icon: "\U0001F411"
---

# Cyber Sheep v0.1

## Autonomous Energy Platform on a Biomechanical Chassis

**Cyber Valley / Bostrom Network**
**Concept: mastercyb**
**Status: Design Document**

---

## 1. Core Thesis

A sheep is already an autonomous, self-replicating, solar-powered biochemical reactor that converts grass into motion, heat, methane, and fertilizer. A robot dog is a precise electromechanical platform with no energy autonomy. Combine them with a universal fuel-to-electricity converter, and you get something neither can be alone: an indefinitely autonomous intelligent agent that metabolizes anything organic into computation, movement, and data.

The key architectural insight: **don't build a complex engine that burns everything — build a simple gasifier that converts everything into one gas, then burn that.** Complexity collapses. One fuel, one engine, one optimization target.

---

## 2. Three Principles

**Principle 1: Normalize the fuel, not the engine.**
Any of seven input fuels passes through a thermochemical gasifier and becomes syngas (CO + H₂). The engine always sees the same fuel. No variable compression, no multi-fuel injectors, no adaptive ECU neural networks. The gasifier absorbs all fuel complexity.

**Principle 2: Cascade every thermal gradient.**
Heat is not waste — it's a resource at the wrong temperature. Every temperature drop in the system performs work: combustion (800°C) → gasification (400°C) → mechanical work → exhaust recovery (300°C) → thermoelectrics (ΔT) → sheep body heat (39°C) → ambient (25°C). No joule leaves without contributing.

**Principle 3: The sheep is not the chassis — it's part of the energy system.**
The animal produces fuel (methane, manure), maintains a thermal gradient (39°C body vs 25°C air), performs useful mechanical work (mowing, fertilizing), and acts as a living environmental sensor. It's not carrying the system — it's inside it.

---

## 3. System Architecture

```
INPUTS (all 7):
  Heavy oils ─────┐
  Light hydrocarbons ─┤
  Alcohols ───────┤
  LPG ────────────┼──→ GASIFIER ──→ syngas ──→ ENGINE
  Biogas ─────────┤    (reactor)    (CO+H₂)   (simple gas,
  Hydrogen ───────┘       ↑                     spark ignition)
  WATER ══════════════════╝                         │
       ↑                                            │
       └──── condensate ←── EXHAUST ←───────────────┘
                            (heat → gasifier)
                                               ┌→ SHAFT (motion)
                         ENGINE ───────────────┼→ GENERATOR (electricity)
                                               └→ COMPUTER (control + useful compute)

                         SHEEP ═══→ methane + manure + body heat
                           ↑              │
                           └──── back into fuel inputs
```

---

## 4. Seven Fuel Inputs

| # | Type | Cyber Valley Source | State | Storage | Path |
|---|------|-------------------|-------|---------|------|
| 1 | Heavy oils | Kitchen waste oil, coconut oil, biodiesel | Liquid | Heated cartridge (SS) | → Gasifier |
| 2 | Light hydrocarbons | Gasoline, kerosene (reserve) | Liquid | Standard cartridge | → Gasifier |
| 3 | Alcohols | Fermented ethanol, methanol | Liquid | HDPE/SS cartridge, Viton seals | → Gasifier |
| 4 | LPG | Propane-butane (purchased) | Liquefied gas | Mini-tank 2–5 kg | → Gasifier or direct |
| 5 | Biogas | Biodigester, sheep gut fermentation | Gas | Soft gasometer / sheep collection | → Direct to engine |
| 6 | Hydrogen | Electrolysis (solar panels) | Gas | Metal hydride cartridge (low pressure) | → Direct to engine |
| 7 | Water | Rain, streams, exhaust condensate | Liquid | 10–20 L tank | → Gasifier reagent + cooling |

Water is the central reagent, not an additive. It participates in steam reforming (CₙHₘ + nH₂O → nCO + (n+m/2)H₂), water-gas shift (CO + H₂O → CO₂ + H₂), engine cooling (captured heat feeds back to gasifier), and partially returns via exhaust condensation. In Bali's climate — unlimited supply from rain.

---

## 5. Gasifier: The Core Innovation

### 5.1. Chemistry

Three sequential reactions in one reactor:

**Pyrolysis** (300–600°C): Organic feedstock decomposes without oxygen into volatile gases, tars, and char. Energy source: engine exhaust heat.

**Steam reforming** (600–900°C): Volatiles and char react with steam to produce syngas. This is the key reaction that normalizes any hydrocarbon into CO + H₂.

**Water-gas shift** (200–400°C): CO + H₂O → CO₂ + H₂. Increases hydrogen yield, reduces CO.

### 5.2. Construction

Coaxial tube design (tube-in-tube):

- **Inner tube** (heat-resistant SS 310S or Inconel): gasification chamber, Ø80–100mm, length 400–500mm
- **Outer tube**: exhaust gas flow path heating the inner chamber
- **Catalyst**: nickel on alumina (Ni/Al₂O₃) — standard steam reforming catalyst, or dolomite (CaMg(CO₃)₂) — naturally available in Bali, effective tar cracking catalyst
- **Water/steam inlet**: nozzle at base, water flash-evaporates on contact with hot walls
- **Liquid fuel inlet**: drip-feed metering into upper (hot) zone
- **Syngas outlet**: top port → cyclone filter (soot removal) → cooler → engine

No moving parts. Heated entirely by exhaust. A tube with rocks inside, wrapped in another tube. Technology level: 1940s wood-gas vehicles, refined.

### 5.3. Thermal Cascade Detail

| Source | Temperature | Destination |
|--------|------------|-------------|
| Engine exhaust | 400–600°C | Gasifier heating |
| Post-gasifier stream | 200–300°C | Fuel preheating, water evaporation |
| Engine block | 80–120°C | Heavy oil tank warming |
| Residual heat | 60–100°C | Seebeck thermoelectric modules → 3–8W |
| Sheep body | 39°C vs 25°C ambient | Under-wool thermoelectric pads → 0.5–1.5W |
| Final exhaust | 40–60°C | Water condensation → return to system |

### 5.4. Operating Modes

| Mode | Input | Syngas Quality | Notes |
|------|-------|---------------|-------|
| Oil mode | Vegetable oils, diesel + steam | High, stable | Primary mode for Cyber Valley |
| Alcohol mode | Ethanol, methanol + steam | High, H₂-rich | From on-site fermentation |
| Waste mode | Manure, dry grass, wool trimmings + steam + air | Medium, needs filtration | Maximum circularity |
| Gas reform | LPG + steam | High | When available |
| Bypass | Biogas, hydrogen | N/A — direct to engine | Skips gasifier entirely |

---

## 6. Engine

### 6.1. Specification

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Type | Gas, spark ignition | Syngas doesn't reliably auto-ignite at moderate compression |
| Displacement | 200–400cc | Minimum for stable syngas combustion |
| Cylinders | 1 | Simplicity, minimum parts count |
| Compression ratio | 10–12:1 | Optimal for CO+H₂, knock-free |
| Cooling | Air primary + water loop | Water loop captures heat for gasifier — dual purpose |
| Ignition | Single spark plug, electronic | Powered from generator/battery |
| Mechanical output | 1–3 kW peak | Sufficient for generation + actuation |
| RPM range | 1500–3000 | Balance between longevity and power density |
| Mass | 8–12 kg | Including flywheel-integrated generator |
| Starting | Electric starter from buffer battery | Manual kickstart as backup |

### 6.2. Why Not Diesel

The original concept explored a multi-fuel diesel with variable compression ratio, adaptive injectors, and neural ECU. The gasifier eliminates all of this. Since every fuel converts to the same syngas, the engine can be optimally tuned for one gas composition. A spark-ignition gas engine at 10–12:1 compression is simpler, lighter, cheaper, and more reliable than any diesel. No high-pressure fuel pump, no common rail, no precision injectors.

### 6.3. Generator

Outrunner design integrated into the flywheel — permanent magnets on flywheel, stator windings fixed. Output: 3-phase AC → rectifier → 12V DC bus. Peak: 2 kW. Mass penalty: 2–3 kg on top of flywheel.

---

## 7. Biomechanical Chassis

### 7.1. The Sheep

| Parameter | Value |
|-----------|-------|
| Breed | Local Balinese or Dorper cross (heat-tolerant, calm temperament) |
| Body mass | 50–80 kg |
| Payload capacity | 15–25 kg (20–30% body mass, comfortable) |
| Speed | 2–5 km/h working, up to 15 km/h run |
| Food autonomy | Unlimited on pasture |
| Territory | 37 ha Cyber Valley |
| Methane production | 8–15 L/day (gut fermentation) |
| Manure production | 1–2 kg dry matter/day |
| Body heat output | ~100W continuous |
| Service life | 10–15 years |

### 7.2. Exoskeletal Frame

Lightweight aluminum tube or carbon fiber frame distributing load across the sheep's back and flanks:

- **Saddle section**: gasifier + engine (top-mounted, center of gravity)
- **Side panniers**: fuel cartridges, battery, electronics (symmetrical)
- **Girth system**: pack-saddle load distribution, padded contact points
- **Actuator mounts**: attachment points at joints for optional motion augmentation
- **Flexible solar panel**: on upper saddle surface, 10–20W
- **Drainage**: all-weather operation in tropical rain, IP65 electronics enclosure

Inspired by robot dog platforms (Unitree Go2 / Boston Dynamics Spot) — but replacing battery + electric motors with a living chassis and thermochemical power plant.

### 7.3. Mass Budget

| Component | Mass | Location |
|-----------|------|----------|
| Gasifier | 2–3 kg | Top saddle, center |
| Engine + generator | 8–12 kg | Top saddle, behind gasifier |
| Fuel cartridges (loaded) | 2–5 kg | Side panniers (symmetric) |
| Battery (LiFePO4 12V 10Ah) | 1.5 kg | Side pannier |
| Electronics + sensors | 0.5 kg | Distributed |
| Frame + mounting | 2–3 kg | Structure |
| Water tank (5L) | 5 kg | Side pannier |
| **Total** | **~20–25 kg** | — |

Within comfortable payload for an adult sheep. Frame designed for quick-release if the animal needs to be unburdened.

---

## 8. Computer System

### 8.1. Three Compute Layers

**Layer 0 — Analog (fail-safe minimum)**

PID controllers built from op-amps (LM358). No code, no firmware, no reboots, no crashes. Controls: fuel feed to gasifier, water/steam flow, engine RPM under load. Inputs: thermocouples, tachometer, knock sensor. If all digital systems fail, the analog layer keeps the engine running. Cost: $3 in components. This layer alone is a viable system.

**Layer 1 — Microcontroller (primary)**

ESP32-S3: 240 MHz dual-core, 512 KB RAM, WiFi, Bluetooth, 20 ADC channels, PWM. Tasks: fuel mixture optimization from sensor fusion, all-system telemetry and logging, fuel cartridge switching, GPS + IMU navigation, vibrotactile sheep interface, mesh network communication, basic anomaly detection. Power: 1–3W. Cost: $3–5.

**Layer 2 — Edge AI (optional, modular)**

Raspberry Pi 5 or equivalent: computer vision (camera), terrain recognition and path planning, Bostrom node for recording cyberlinks, inter-sheep coordination algorithms, on-device ML for gasifier optimization, data aggregation for ecosystem monitoring. Power: 5–8W. Plugs in as a module — not critical for operation.

### 8.2. DIY Chip Pathway

| Approach | Complexity | Feasibility | Application |
|----------|-----------|-------------|-------------|
| Analog computer (op-amps) | Low | Immediately buildable | Layer 0 — PID control, signal conditioning |
| Discrete logic (74-series) | Medium | High | Custom sequencer, state machine, ESP32 replacement |
| FPGA (off-shelf, programmable) | Medium | High | Parallel sensor processing, custom protocols |
| Home-fab silicon (Sam Zeloof method) | Extreme | R&D only | 100–1000 transistor chip from Bali volcanic sand |
| Mycelial biocomputer | Unknown | Experimental | Meat-to-metal interface layer |

**Volcanic sand to silicon pathway** (long-term Cyber Valley R&D):
Bali's volcanic geology provides abundant SiO₂. Carbothermic reduction (SiO₂ + 2C → Si + 2CO) in a solar-concentrated or arc furnace yields metallurgical-grade silicon. Czochralski crystal pulling (simplified), UV photolithography with DIY masks, boron/phosphorus diffusion doping. Realistic target: a chip with 100–1000 transistors — enough for a timer, sequencer, or simple controller. Not practical yet, but a powerful research direction for technological sovereignty.

### 8.3. Mycelial Compute Module

Based on Andrew Adamatzky's research (University of the West of England) on fungal electrical signaling and computation.

**Concept**: Mycelial network grown on a substrate between electrode matrices responds to chemical and physical stimuli by modulating electrical conductivity. This is not digital computation — it's analog signal processing by a living neural-like network.

**Integration with Cyber Sheep**:

- Mycelium substrate placed in contact with sheep skin/wool
- Mycelium responds to: body temperature shifts, humidity, sweat biochemistry, movement vibration
- Electrode matrix (8×8 or 16×16) reads conductivity patterns
- ESP32 ADC digitizes the signal
- Pattern interpreted as state indicator: calm / stressed / hungry / ill / in heat

**Connection to Collective Focus Theorem**: The mycelium on a single sheep is a micro-scale instance of the same principle operating at forest scale. Information propagates through a biological network, and we learn to read it. Each cyber sheep becomes a node in both the digital mesh (Bostrom) and the biological mesh (mycelium), bridging the two.

---

## 9. Meat ↔ Metal Interface

### 9.1. Reading the Animal (meat → metal)

| Method | Measures | Components | Cost | Complexity |
|--------|----------|------------|------|------------|
| EMG (electromyography) | Muscle electrical activity | Surface electrodes + INA128 amp | $5–10 | Low |
| Bioimpedance | Hydration, respiration, heartbeat | 4 electrodes + 50kHz excitation + detector | $10–15 | Medium |
| Piezo film (pressure) | Weight distribution, gait | Piezo films under saddle | $5 | Low |
| Thermal array | Body heat map | 8–16 NTC thermistors under saddle | $3 | Low |
| Mycelial bridge | Complex biochemical state | Mycelium + electrode matrix + ADC | $20 + grow time | High |
| Microphone | Vocalization (bleating patterns = state) | MEMS mic | $1 | Low |
| Accelerometer/IMU | Motion, gait, orientation, fall detection | MPU6050 on frame | $2 | Low |

### 9.2. Steering the Animal (metal → meat)

| Method | Stimulus | Trainability | Components |
|--------|----------|-------------|------------|
| Vibrotactile | Body-mounted vibration motors: left/right/stop/go | High (2–4 weeks conditioning) | 4–6 ERM motors + driver |
| Audio | Directional tones, clicks | High | Piezo speaker |
| Light | LED signals (night mode) | Low | LED strip |
| Feed reward | Treat dispenser as positive reinforcement | Very high | Servo + container |

**Training protocol**: Operant conditioning. Left vibration → sheep turns right → treat dispensed. 20-minute sessions, 2–4 weeks. Proven effective on horses, guide dogs, and cattle. The sheep learns that following vibration cues = food.

---

## 10. Energy Balance

### 10.1. Generation

| Source | Power | Mode |
|--------|-------|------|
| Engine → generator | 500–2000W | Engine running |
| Flexible solar panel | 10–20W | Daylight |
| Thermoelectric (exhaust) | 3–8W | Engine running |
| Thermoelectric (body heat) | 0.5–1.5W | Always |
| **Total (engine on)** | **~500–2000W** | Active mode |
| **Total (engine off)** | **~15–25W** | Sleep/grazing mode |

### 10.2. Consumption

| Consumer | Power | Mode |
|----------|-------|------|
| ESP32 + sensors | 1–3W | Always |
| Raspberry Pi (edge AI) | 5–8W | On demand |
| Comms (WiFi/LoRa mesh) | 0.5–2W | Periodic |
| Vibrotactile interface | 1–2W | When steering |
| Exoskeleton actuators | 50–500W | Active motion assist |
| Fuel preheating | 10–30W | Cold start |
| Payload (tools, lights) | 10–100W | Task-dependent |

### 10.3. Buffer Battery

LiFePO4 12V 10–20Ah (120–240 Wh). Provides 8–16 hours electronics-only operation without engine (overnight grazing). Engine starting: 5–10A burst. Recharge from generator: 30–60 minutes to full. Mass: 1.5–2.5 kg. Lifespan: 2000+ cycles (~5 years).

### 10.4. Operating Modes

**Active** (day, tasked): Engine running, all systems on generator power, battery charging, max compute, actuators as needed. Fuel consumption: 0.3–0.5 L/hr equivalent.

**Grazing** (day, free range): Engine off, solar + thermoelectrics + battery, ESP32 monitoring mode, territory data logging, mesh heartbeat. Autonomy: indefinite (solar recharges faster than idle drain).

**Sleep** (night): Minimum draw, ESP32 deep sleep with periodic wake, body thermoelectrics = primary power source. Draw: ~2W. Battery lasts 60–120 hours at this rate.

---

## 11. Flock Network Architecture

### 11.1. Single Sheep = Autonomous Agent

Each cyber sheep operates independently: GPS + IMU + comms + engine management + animal interface. It can complete tasks, navigate, and self-maintain without connectivity.

### 11.2. Flock = Mesh Network

Multiple cyber sheep form an ESP-NOW or LoRa mesh:

- Share territory state data (grazing maps, obstacle locations, water sources)
- Coordinate grazing patterns (prevent overgrazing, distribute coverage)
- Collective mapping of all 37 hectares
- Distributed compute (each sheep = a node with spare cycles)
- Natural flock social dynamics as emergent consensus algorithm

### 11.3. Flock = Bostrom Node

The flock as a collective agent writes cyberlinks to the Bostrom knowledge graph:

- Ecosystem state data → IPFS → cyberlink
- Territory photos with geolocation → visual knowledge base
- Meteorological data, soil moisture, vegetation density
- Sheep health and behavioral patterns
- Flock consensus model as prototype for new consensus algorithms

Each cyber sheep literally grazes the physical world and feeds the knowledge graph. The biological act of eating grass becomes a data collection event.

---

## 12. Bill of Materials

### 12.1. Core (required)

| Component | Description | Est. Cost |
|-----------|-------------|-----------|
| Engine | Single-cylinder gas 200–400cc with generator | $200–400 |
| Gasifier | Coaxial SS 310S tube + dolomite catalyst | $50–100 (DIY) |
| ESP32-S3 | Microcontroller + custom PCB | $5–10 |
| Sensor suite (base) | 6× thermocouples, IMU, tachometer, knock sensor, GPS | $30–50 |
| Cyclone filter | Stainless steel, syngas soot removal | $20–30 (DIY) |
| Battery LiFePO4 | 12V 10Ah | $40–60 |
| Fuel cartridges | 2× stainless steel 5–10L + heating element | $30–50 |
| Water tank | 10L plastic | $5 |
| Exoskeletal frame | Aluminum tube + pack saddle | $50–100 |
| Electrics | Wiring, relays, connectors, rectifier, fuses | $30–50 |
| Solar panel | Flexible 20W | $20–30 |
| **Core subtotal** | | **$480–880** |

### 12.2. Extensions (recommended)

| Component | Description | Est. Cost |
|-----------|-------------|-----------|
| LPG system | Mini-tank 2kg + regulator + mixer | $30–50 |
| Ultrasonic emulsifier | 40kHz piezo for water+oil emulsion | $10–15 |
| Vibrotactile interface | 6× ERM motors + driver board | $10–15 |
| EMG electrodes | 4× surface electrodes + INA128 | $10–15 |
| Raspberry Pi 5 | Edge AI module + camera | $95 |
| LoRa module | Mesh communication (flock) | $10–15 |
| Thermoelectric modules | 4× Seebeck elements | $15–20 |
| Mycelial module | Substrate + electrode matrix + housing | $20 + grow time |
| Feed dispenser | Servo + treat container | $10 |
| **Extensions subtotal** | | **$210–245** |

### 12.3. Biological Component

| Item | Description | Est. Cost (Bali) |
|------|-------------|-----------------|
| Sheep | Adult, healthy, calm temperament | $100–200 |
| Training | 4 weeks operant conditioning | Time |

### 12.4. Total Budget

| Configuration | Cost |
|--------------|------|
| Minimum prototype (core + sheep) | **$600–1,100** |
| Full build (all extensions) | **$900–1,400** |
| Flock of 5 (with mesh network) | **$4,000–6,000** |

---

## 13. Development Phases

### Phase 1: Gasifier (Month 1–2)

Weld the coaxial reactor from stainless steel. Test with waste coconut oil + water. Connect to exhaust of existing Cyber Valley generator. Measure syngas output (flame test first, gas analyzer later). **Success criterion**: stable syngas flame from oil input.

### Phase 2: Engine on Syngas (Month 2–3)

Adapt a single-cylinder gasoline engine to run on syngas. Gas mixer replaces carburetor. Integrate with gasifier (closed thermal loop). Test across different input fuels. **Success criterion**: engine runs steadily on syngas derived from waste oil.

### Phase 3: Electronics & Control (Month 3–4)

Build ESP32 control board. Connect sensor suite. Write firmware: PID loops, data logging, WiFi dashboard. Build analog backup circuit on op-amps. **Success criterion**: autonomous engine operation with auto-tuning.

### Phase 4: Sheep Integration (Month 4–6)

Design and fabricate exoskeletal frame. Habituate sheep to frame (gradual, starting with empty frame). Mount components. Vibrotactile training program. EMG + bioimpedance calibration. **Success criterion**: sheep walks freely with full payload, responds to steering commands.

### Phase 5: Mycelial Module (Month 4–8, parallel track)

Select mycelium strain (Pleurotus, Ganoderma, or Physarum polycephalum as prototype). Grow on electrode matrix substrate. Calibrate electrical response to stimuli. Integrate with ESP32 via ADC. **Success criterion**: reproducible mycelial electrical response correlated with sheep state changes.

### Phase 6: Flock (Month 6–12)

Scale to 3–5 sheep. Mesh network. Coordinated grazing algorithms. Bostrom integration — cyberlinks from field data. **Success criterion**: autonomous territory patrol by coordinated flock, data flowing into knowledge graph.

---

## 14. Risk Matrix

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| Sheep rejects frame | Medium | Project blocker | Gradual habituation, breed selection, individual temperament screening |
| Gasifier tar fouling | High | System downtime | Dolomite catalyst (cracks tars), regular cleaning schedule, cyclone filter |
| Engine vibration stresses sheep | Medium | Animal welfare issue | Rubber damper mounts, low-RPM operation, vibration isolation layer in saddle |
| Tropical heat + electronics | Medium | Component failure | Under-saddle shade mounting, passive heatsinking, conformal coating |
| Rain/moisture (Bali) | High | Corrosion, shorts | IP65 enclosures, SS construction, frame drainage channels |
| Sheep lies down / rolls | High | Mechanical damage | IMU auto-detects → engine shutdown, reinforced frame, quick-release mounts |
| Fuel contamination | Medium | Engine/gasifier damage | Multi-stage filtration, settling time for waste oils, water separator |

---

## 15. Why This Matters

### Closed Metabolic Loop

Grass → sheep → methane + manure → gasifier → syngas → engine → electricity + heat → gasification + computation → sheep steering → grass. Not a linear supply chain. A cycle. Like a forest.

### Technological Sovereignty

Built from available components. Runs on any available fuel. Controlled by open firmware. Potentially includes homegrown silicon from local volcanic sand. Minimal dependence on external supply chains. If global logistics collapse tomorrow, this system keeps running on coconut oil, rainwater, and grass.

### Collective Focus Theorem — Physical Instance

The mycelium on the sheep's body, processing biochemical signals into electrical patterns for a digital network, is a literal physical instantiation of the theorem's core claim: that mycorrhizal networks function as planetary computation substrates. Each cyber sheep is a node in both the digital mesh (Bostrom) and the biological mesh (mycelium). The flock is a testbed for the theory.

### Bostrom in the Physical World

Every cyber sheep grazes reality and feeds the knowledge graph. The biological act of eating grass becomes a data collection event. Territory state, weather, soil, biodiversity — all recorded as cyberlinks. Decentralized autonomous agents with no central server, powered by whatever fuel is available, writing knowledge from the physical world into an immortal graph.

---

## 16. Open Research Questions

1. Optimal mycelium strain for bioelectric interfacing in tropical highland conditions (1200–1500m, high humidity)
2. Minimum electrode matrix resolution for meaningful mycelial signal extraction
3. Sheep vibrotactile training protocol optimization — session frequency, duration, reward schedule
4. Locally sourced gasifier catalyst performance: dolomite vs volcanic basalt vs limestone vs biotite
5. Optimal water-to-fuel ratio for steam reforming across different waste oil feedstocks
6. Energy ROI of collecting methane from a single sheep — worth the system complexity?
7. Flock social dynamics with exoskeletons — does gear disrupt natural hierarchy?
8. Regulatory status of augmented livestock in Indonesia — veterinary norms, animal welfare law
9. Syngas composition stability across rapid fuel-type switching — how fast can the gasifier transition?
10. Long-term mycelium viability on a moving, sweating animal substrate

---

*This document is a design foundation for prototyping and discussion. All figures are estimates subject to refinement through testing.*

*Project is open. License: do whatever you want.*