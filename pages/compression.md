---
tags: computer science, information theory
crystal-type: process
crystal-domain: mathematics
---

Reducing data size by eliminating redundancy. Encoding information in fewer bits than the original representation.

## lossless

Every bit of original data is perfectly recoverable.
- Huffman coding: variable-length codes based on symbol frequency
- Lempel-Ziv (LZ77, LZ78, LZW): dictionary-based, replacing repeated patterns with references. Used in gzip, PNG
- arithmetic coding: encoding entire messages as single numbers in [0,1)
- run-length encoding: replacing consecutive identical symbols with count + symbol

## lossy

Discards information deemed less perceptible. Smaller files, irreversible.
- JPEG: discrete cosine transform, quantization of high-frequency components in images
- MP3/AAC: psychoacoustic models, discarding sounds humans perceive poorly
- H.264/H.265: video compression, motion estimation, inter-frame prediction

## theoretical limit

Shannon entropy defines the minimum average bits per symbol for lossless compression. No [[algorithms|algorithm]] can compress below this bound. Information content is incompressible.

## applications

- storage: fitting more data on disk, [[databases]] use compression internally
- transmission: faster transfer over [[networks]], reduced bandwidth cost
- [[IPFS]]: content-addressed storage benefits from deduplication, a form of compression
- [[cyber]]: compressed representations in the [[knowledge graph]]

## connections

Rooted in [[information theory]] and Shannon entropy. [[encryption]] and compression interact: encrypted data is incompressible (maximum entropy). [[algorithms]] for compression are studied in [[complexity theory]].