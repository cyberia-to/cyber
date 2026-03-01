---
tags: computer science, machine learning
crystal-type: entity
crystal-domain: computer science
---
Layers of weighted connections that learn to approximate functions from data. The computational substrate of modern artificial intelligence.

## architecture

neuron: weighted sum of inputs passed through an activation function (ReLU, sigmoid, tanh)

layer: collection of neurons. Input, hidden, and output layers.

feedforward: signals flow in one direction, universal approximation theorem

recurrent (RNN): connections loop back, modeling sequences, LSTM and GRU variants

convolutional (CNN): local receptive fields, weight sharing, dominant in vision

transformer: self-attention mechanism, parallelizable, dominant in language. GPT, BERT

## learning

backpropagation: computing gradients of the loss function via chain rule, flowing error backward through layers

gradient descent: adjusting weights to minimize loss (SGD, Adam)

loss function: measures the gap between prediction and ground truth

overfitting and regularization: dropout, weight decay, early stopping

## deep learning

Stacking many layers enables hierarchical feature extraction. Deeper networks learn increasingly abstract representations. Scale of data, compute, and parameters drives capability.

## connections

[[algorithms]] underpin training optimization. [[data structures]] (tensors, graphs) organize network computation. [[type theory]] informs tensor shape checking. [[consensus algorithms]] in [[cyber]] enable decentralized training and inference.