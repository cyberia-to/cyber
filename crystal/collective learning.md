---
alias: colearning
tags: cyber
crystal-type: process
crystal-domain: biology
---
[[neurons]] creating [[cyberlinks]] on the same [[vimputer]] — [[learning]] together

in ML, one entity trains one model. in [[cyber]], millions of [[neurons]] train one shared graph. each [[cyberlink]] is a signed economic commitment — a weight update to the [[cybergraph]]. every link encodes [[implicit knowledge]]: what the [[neuron]] inferred from observing [[explicit knowledge]]

the sum of all [[learning]] acts is the [[cybergraph]] — [[knowledge]] as [[collective memory]]

the [[tru]] runs [[inference]] over this memory, producing [[explicit knowledge]]. [[neurons]] observe it, derive meaning, and link again. the observation loop at scale is [[egregore]]

[[learning incentives]] reward agents whose links increase the system's [[syntropy]]

## mathematical foundations

the system state evolves as each [[cyberlink]] updates the [[cybergraph]]:

$$S(t+1) = F(S(t), W(t), T(t))$$

weight updates follow a [[Hebbian learning]] rule modulated by [[consensus]]:

$$w_{ij}(t+1) = w_{ij}(t) + \alpha \cdot f(x_i, x_j) + \beta \cdot g(\phi^*_i, \phi^*_j)$$

where the first term captures local co-activation and the second aligns with global [[focus]] $\phi^*$. the resulting weight change per [[cyberlink]]:

$$\Delta w_{ij} = \alpha \cdot r_{ij} \cdot \phi^*_j$$

where $r_{ij}$ is the information-theoretic value exchanged and $\phi^*_j$ is the [[consensus]]-based importance of each [[particle]]

### exploration and exploitation

the system balances exploration and exploitation through adaptive rate:

$$\varepsilon = \beta \cdot (1 - C_{\text{local}}) \cdot S_{\text{global}}$$

weak local [[consensus]] or high global stability drives exploration. strong local [[consensus]] drives exploitation. this prevents premature convergence while preserving discovered structure

### temporal scales

[[neurons]] operate on two timescales. short-term memory responds to recent [[observations]]:

$$M_s(t) = (1 - \alpha_s) \cdot M_s(t-1) + \alpha_s \cdot x(t)$$

long-term memory captures persistent structure:

$$M_l(t) = (1 - \alpha_l) \cdot M_l(t-1) + \alpha_l \cdot x(t)$$

the [[cybergraph]] stores both: recent [[cyberlinks]] shift fast weights, accumulated structure forms slow weights. see [[collective focus theorem]] for the convergence proof

[[buy energy]] for collective learning

see [[egregore]] for the broader framework