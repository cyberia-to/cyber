---
tags: cyber, language
alias: Wav, wave language, signal language
crystal-type: entity
crystal-domain: cyber
---
the [[signal]] language. a [[signal]] is a waveform — a continuous function sampled at discrete points

| Op | Action |
|---|---|
| `fft(x)` | Fast [[fourier transform]] |
| `ifft(X)` | Inverse FFT |
| `convolve(a, b)` | Convolution of two signals |
| `lowpass(x, cutoff)` | Low-pass filter |
| `resample(x, rate)` | Resample to new rate |
| `correlate(a, b)` | Cross-correlation |
| `energy(x)` | Signal energy |
| `peak_detect(x)` | Find peaks |

use cases: sensor data processing, audio, seismic, environmental monitoring. and — critically — [[Goldilocks homomorphic encryption]]: polynomial multiplication in R_q = Z_q[X]/(Xⁿ+1) is negacyclic convolution of coefficient [[vector]]s. the same butterfly network, the same NTT engine. Wav is the language of sensing at all scales — from physical waveforms to encrypted computation over polynomial rings. FHE is not a separate concern; it is sensing at the algebraic level, where the [[signal]] is a ciphertext. Wav extended with noise budget types becomes the FHE compiler: noise tracking, modulus ladder management, PBS scheduling. compiles to [[Trident]] for [[proof]] of correct R_q operations

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture