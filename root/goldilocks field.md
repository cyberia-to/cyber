---
tags: cyber, mathematics, cryptography
alias: Goldilocks, goldilocks, F_p, prime field
crystal-type: entity
crystal-domain: cyber
---
# goldilocks field

the finite field $\mathbb{F}_p$ where

$$p = 2^{64} - 2^{32} + 1$$

the arithmetic foundation of [[cyber]]. every [[stark]] proof, every [[nox]] computation, every [[Hemera]] hash operates over this field

## why Goldilocks

the name comes from the prime being "just right" for 64-bit hardware. three properties make $p$ exceptional:

1. fits in a single 64-bit register — no multiprecision arithmetic needed
2. multiplication reduces via a single shift-and-subtract: $2^{64} \equiv 2^{32} - 1 \pmod{p}$, so reducing a 128-bit product requires only cheap 32-bit operations
3. admits efficient NTT (Number Theoretic Transform): the multiplicative group has order $p - 1 = 2^{32} \cdot (2^{32} - 1)$, providing $2^{32}$-th roots of unity for FFT-based polynomial multiplication

the result: field arithmetic runs at near-native CPU speed. modular addition costs one comparison and one conditional subtraction. modular multiplication costs one 64-bit multiply plus a structured reduction. no Montgomery form, no Barrett reduction, no bignum library

## the arithmetic

field elements are integers in $\{0, 1, \ldots, p-1\}$. standard operations:

$$a + b \pmod{p}, \quad a \cdot b \pmod{p}, \quad a^{-1} = a^{p-2} \pmod{p}$$

the structured reduction exploits the sparse form of $p$:

$$x \bmod p: \quad x = x_{\text{hi}} \cdot 2^{64} + x_{\text{lo}} \implies x \equiv x_{\text{lo}} + x_{\text{hi}} \cdot (2^{32} - 1) \pmod{p}$$

one 64-bit multiply, one shift, one subtraction. this is why Goldilocks outperforms generic primes by 3-5x on commodity hardware

## role in cyber

the Goldilocks field is the single numeric type in [[nox]]. where [[Nock]] uses natural numbers and decrement, nox uses field elements and field inverse. this means:

- every [[nox]] program is a sequence of field operations
- every execution trace is a matrix over $\mathbb{F}_p$
- every [[stark]] constraint is a polynomial equation over $\mathbb{F}_p$
- [[Hemera]] hashes field elements directly — no serialization overhead

the alignment between the VM, the proof system, and the hash function eliminates representation mismatches. there is one numeric domain from application logic through to cryptographic verification

## extensions

for operations requiring more security bits, [[cyber]] uses quadratic and cubic extensions:

$$\mathbb{F}_{p^2} = \mathbb{F}_p[x] / (x^2 + 1), \quad \mathbb{F}_{p^3} = \mathbb{F}_p[x] / (x^3 - x - 1)$$

extension arithmetic preserves the speed advantage: each extension multiply decomposes into a small constant number of base field multiplies. [[Brakedown]] and [[Hemera]] use extensions where the security proof demands a larger evaluation domain

see [[stark]] for the proof system built on this field. see [[Hemera]] for the hash function. see [[nox]] for the virtual machine. see [[Goldilocks field processor]] for dedicated hardware. see [[trident]] for how field arithmetic scales to AI workloads
