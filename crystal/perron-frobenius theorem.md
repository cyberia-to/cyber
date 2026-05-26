---
tags: mathematics, cyber
crystal-type: entity
crystal-domain: cyber
alias: Perron-Frobenius, perron-frobenius
---
the theorem guaranteeing that an irreducible non-negative matrix has a unique largest real eigenvalue with a strictly positive eigenvector

mathematical foundation of the [[collective focus theorem]]: ensures a unique [[focus]] distribution $\phi^*$ exists for the [[cybergraph]]

statement: let $A$ be an irreducible non-negative square matrix. then $A$ has a real eigenvalue $\lambda_1 > 0$ such that $\lambda_1 \geq |\lambda|$ for every eigenvalue $\lambda$ of $A$. the eigenvector corresponding to $\lambda_1$ has all strictly positive entries

in [[cyber]], the transition matrix of the [[cybergraph]] satisfies irreducibility when the graph is strongly connected. the [[tru]] enforces this by adding a teleportation term (damping factor), guaranteeing convergence of [[diffusion]] to a unique stationary distribution

this is the same theorem that underlies PageRank. cyber generalizes the result through the [[tri-kernel]]: [[diffusion]] uses the Perron-Frobenius eigenvector directly, while [[springs]] and [[heat]] operate on the graph Laplacian spectrum

convergence rate depends on the spectral gap $\lambda_1 - |\lambda_2|$: larger gap means faster consensus on [[relevance]]

see [[collective focus theorem]], [[focus]], [[diffusion]], [[cybergraph]], [[graph theory]], [[tri-kernel]]
