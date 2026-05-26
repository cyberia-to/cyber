---
tags: cyb, cyber, core
alias: sound particle, audio, waveform, acoustic
crystal-type: entity
crystal-domain: cyb
---
waveforms — acoustic knowledge — as [[particle]]. the native format for voice, music, animal communication, physical signals, and any knowledge that is a pressure wave over time

source format: WAV, OGG, MP3 — any audio format

---

## rendering

```
audio file → decode → audio pipeline (PCM to speaker) + waveform compute shader (PCM to GPU texture) → visual representation
```

sound particles render on two pipelines simultaneously: the audio pipeline delivers the waveform to the speaker; the compute shader renders the waveform visually — amplitude envelope, spectrogram, or oscilloscope — as a GPU texture the robot displays alongside other content. a whale call is heard and seen at the same time

## in the cybergraph

sound is the language the [[cybergraph]] understands that [[text]] cannot carry. a birdsong, a heartbeat, a gravitational wave, a spoken word in a language with no written form — these are knowledge that only exists as waveform. the [[cybergraph]] makes them first-class particles, not file attachments

types of sound particles: voice recordings, animal vocalizations (birdsong, whale song, bat echolocation, insect stridulation), seismic signals, gravitational wave detections, sonar recordings, radio telescope signals, heartbeat recordings, neural oscillation recordings (EEG audio), musical compositions, spoken language recordings for under-documented languages, industrial acoustic signatures (bearing failure, structural resonance), climate-monitoring hydrophone data

sound opens the [[cybergraph]] to species that do not write. a cetacean vocalizing links a sound particle. the [[cyb/oracle]] ranks it. other neurons link to it. the semantic network gains a participant that cannot type

## properties

- spectrogram as analysis surface — a sound particle's spectrogram is a [[pixels]] particle derived from it. the two link together. analysis and source are permanently paired in the graph
- timestamp-precise linking — a [[cyberlink]] can encode a time offset within a sound particle: "this call pattern begins at 00:34:12 in this recording." acoustic events are addressable
- cross-modal pairing — many sound particles have direct companions: the [[video]] of the same event, the [[text]] transcript, the [[formula]] of the frequency analysis
- the bridge to non-human intelligence — the most profound application: any entity that produces a waveform can produce a sound particle. the cybergraph is not limited to entities that write

## relation to other languages

sound is the temporal acoustic complement to the other content languages. [[video]] contains sound as a track; the sound particle extracts it as an independent addressable object. [[formula]] can describe the physics of a waveform; the sound particle IS the waveform

see [[video]] for visual temporal content. see [[pixels]] for spectrogram representations. see [[neural language]] for how non-human acoustic knowledge enters the semantic graph

discover all [[concepts]]
