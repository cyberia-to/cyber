---
tags: article
crystal-type: pattern
alias: unspent transaction output
---

# UTXO

The unspent transaction output model, introduced by [[Bitcoin]], where each transaction consumes previous outputs and creates new ones.

Every UTXO represents a discrete, spendable coin locked by a script. A transaction proves authorization by satisfying the locking conditions of its inputs, then produces fresh outputs for recipients.

The global state is the set of all unspent outputs — there are no account balances, only coins waiting to be spent. Validation requires checking that inputs exist and are unspent, and that the sum of inputs covers the sum of outputs plus [[gas]].

This model enables simple parallelism: transactions touching disjoint UTXO sets can be validated independently. [[Cardano]] and [[Ergo]] extend the pattern with programmable scripts on each output.

discover all [[concepts]]
