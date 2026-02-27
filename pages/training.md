---
alias: train
tags: cyber, core
crystal-type: process
crystal-domain: biology
---
the ML approximation of [[learning]]. one-directional: data goes in, model weights come out

in machine learning, training updates model weights from data. the model does not observe its own output mid-training. training ends, then [[inference]] begins. they are sequential phases

in [[cyber]], [[learning]] updates the [[cybergraph]] by adding [[cyberlinks]]. the graph IS the model. every [[cyberlink]] is a weight update. at this level — a single link — the analogy holds

| | machine learning | [[cyber]] |
|---|---|---|
| process | training | [[learning]] |
| who | gradient descent | [[neurons]] |
| model | neural network weights | [[cybergraph]] |
| data | training set | [[particles]] |
| cost | compute (FLOPs) | [[focus]] |
| result | trained model | [[knowledge]] |

the analogy breaks at the loop. in ML, the model does not observe its own inference and retrain itself in a continuous cycle. in [[cyber]], every [[learning]] act is a response to the [[tru]]'s [[inference]] — a [[neuron]] observes [[explicit knowledge]], derives [[implicit knowledge]], and links. [[learning]] and [[inference]] are concurrent, interleaved, continuous. training captures the write operation but misses the observation loop that makes [[learning]] a living process

the key difference: in ML, one entity trains one model. in [[cyber]], millions of [[neurons]] train one shared graph. this is [[collective learning]]

discover all [[concepts]]
