---
tags: cyb, cyber, core
alias: pixels particle, image, raster, photograph
crystal-type: entity
crystal-domain: cyb
---
captured reality as [[particle]]. the native format for photographs, satellite imagery, microscopy, medical scans, and any content that is a grid of color values

source format: PNG, WebP, JPEG — any raster image format

---

## rendering

```
image file → decode → GPU texture upload → sampler → fragment shader
```

pixels particles upload as GPU textures and sample through fragment shaders. mip-mapping for downscaling. trilinear filtering for smooth zooming. HDR where the display supports it. the robot renders a 100-megapixel satellite image and a 16x16 icon through the same pipeline

## in the cybergraph

pixels is how the physical world enters the [[cybergraph]]. a photograph is evidence. a satellite image is data. a microscopy slide is an observation. when they become particles, they become linkable, rankable, and permanent by axiom A3

types of pixels particles: photographs, satellite images, electron microscopy, medical scans (MRI, CT, X-ray, PET), telescope images, aerial photography, specimen documentation, crime scene evidence, architectural renderings, artwork, screenshots, maps (raster tiles), thermal imagery, spectrometry outputs

a pixels particle is often the most irreplaceable in the graph: it is the observation itself. the text particle describes the experiment; the pixels particle IS the specimen. the formula predicts the result; the pixels particle IS the result

## properties

- content-addressed — the CID of a pixels particle is derived from its exact content. the same image anywhere in the world has the same CID. duplication is structurally impossible
- verifiable provenance — a pixels particle linked by a [[neuron]] at a known block height is timestamped evidence. falsification requires breaking the hash
- annotatable — the [[cybergraph]] supports spatial annotation: a [[cyberlink]] can encode a bounding box or polygon reference within a pixels particle, making region-level linking possible
- composable — pixels particles compose inside [[component]] particles. a medical viewer is a component that renders a sequence of pixels particles (scan slices) as an interactive 3D volume

## relation to other languages

pixels is the ground truth. [[vector]] draws what must be understood; pixels captures what is. [[video]] extends pixels into the time dimension. [[text]] annotates what pixels shows; pixels shows what [[text]] describes

see [[vector]] for resolution-independent imagery. see [[video]] for temporal sequences. see [[sound]] for the acoustic complement to visual evidence

discover all [[concepts]]
