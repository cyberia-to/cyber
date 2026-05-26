---
tags: cyber
crystal-type: pattern
crystal-domain: cybics
alias: predictive processing
---
the brain as a prediction machine — perception is not passive observation but active inference about the causes of sensory signals

the cortex maintains a hierarchical generative model. each layer predicts the activity of the layer below. only prediction errors propagate upward. learning adjusts the model to minimize these errors

## the architecture

- top-down: predictions flow down the [[hierarchy]]
- bottom-up: prediction errors flow up
- lateral: [[precision]] weights modulate which errors matter

the system converges when predictions match observations — [[free energy]] is minimized. what remains is the model's best explanation of the world

## connection to active inference

predictive coding is the neural implementation of [[active inference]]:

- perception: update predictions to reduce sensory errors (change the model)
- action: move to reduce proprioceptive errors (change the world)
- [[attention]]: adjust [[precision]] to weight errors by confidence (change the gain)

[[Karl Friston]] showed these are all gradient descent on the same [[free energy]] functional

## in cyber

the [[cybergraph]] implements a distributed version:

- each [[neuron]] predicts local [[focus]] distribution based on its model of the graph
- [[cyberlinks]] that match predictions (confirm structure) are low-error
- [[cyberlinks]] that violate predictions (novel connections) are high-error — and potentially high-reward if they reduce [[free energy]] globally

see [[active inference]] for the framework. see [[free energy principle]] for the theory. see [[precision]] for the weighting mechanism