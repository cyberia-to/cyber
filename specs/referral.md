---
tags: cyber, specs, money, cybernomics
crystal-type: spec
crystal-domain: cyber
alias: referral, referral system, referrer share, dunbar decay
status: draft
---

# referral

growth is work, and the protocol pays for work: a bind-once referrer earns a share of every settled reward of its referee. the share is the 10% pool [[li]] already reserves; the decay makes it honest at scale; the witness gate makes it [[sybil-proof]].

## the curve

parameters in micros: pool $P = 100\,000$ (10%), floor $F = 10\,000$ (1%), Dunbar scale $D = 150$, activity window $W = 30$ epochs.

$$s(n) = \max\!\left(F,\ \frac{P \cdot D}{D + 9n}\right)$$

$n$ — direct referees active within $W$. the slope $9 = P/F - 1$ is forced by the boundary conditions $s(0) = P$ and $s(D) = F$; beyond $D$ the share stays at the floor. the referrer cut of a settled reward $A$ is $\lfloor A \cdot s(n)/10^6 \rfloor$; the referee keeps the rest.

hyperbolic, not linear: income $n \cdot s(n)$ must rise with every referee — the percentage falls, the volume grows, nobody is paid to stop recruiting. a linear descent to the floor peaks at $n \approx 83$ and then pays less for each newcomer; on the hyperbola each active referee adds at least the floor, and in integer micros monotonicity survives quantization for all $n \le D$. a large node lives on volume, a newcomer on margin, and Dunbar is the equilibrium cell size rather than a limit: past $D$ growth continues by delegation — new people head their own branches.

## the witness gate

decay alone pays for splitting: a referrer at $n = 150$ earns $150F$ per unit of mean referee reward, two fresh identities at 75 each earn $2 \cdot 75 \cdot s(75) \approx 1.82\times$ more. so the curve opens only to a witnessed identity — one [[attested genome protocol|nullifier]], one curve — and an unwitnessed referrer earns the flat floor at any $n$. splitting across unwitnessed identities returns exactly the floor; splitting across witnessed identities requires distinct humans, which is recruiting — the attack becomes the behavior the pool buys. the curve inherits its security from the attestation primitive, the way reward inherits sybil-resistance from karma non-transferability in [[tru]].

[[moon passport]] resolves names, not persons — one owner holds many passports — so the gate mounts on the passport's proof slot, not on the passport itself.

## wiring

binding. the referrer arrives in boot.dat ([[cyb-boot]]) and binds on first sync. bind-once: self-binding, rebinding, and upline cycles are rejected.

payout. every settle path converges in `apply_settle_receipt` ([[cyb]] core): the epoch's [[Shapley value|Shapley]] share splits — the referee is marked active, the referrer cut is credited with a matching [[tok]] mint leg (cut + net = share, conservation), the net mints to the referee under clock-B escrow. [[sense]] and sigma read `ReferralAccrued`.

witness. `attest_witness` admits a neuron to the curve; the verifying implementation — a [[moon passport]] extension proof or a genome nullifier — is the mount point, trusted-local until it lands.

## open

- destination of the freed margin $P - s(n)$: treasury, or cashback to the referee — a counter-pull that equilibrates branches around $D$
- turnover-driven decay instead of the activity window
- referrer-side clock-B escrow

see [[specs/money-loop|money loop]] for the settle path · [[attested genome protocol]] for the witness primitive

discover all [[concepts]]
