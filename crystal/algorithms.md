---
tags: computer science
crystal-type: entity
crystal-domain: computer science
---

A step-by-step procedure that transforms input into output in a finite number of operations. The foundation of all [[computation]].

## complexity

[[Big-O notation]] measures how an algorithm's time or space grows with input size. Common classes: O(1) constant, O(log n) logarithmic, O(n) linear, O(n log n) linearithmic, O(n^2) quadratic, O(2^n) exponential. Closely tied to [[complexity theory]].

## core families

- sorting: bubble, merge, quick, heap -- ordering elements by key
- searching: binary search, depth-first, breadth-first
- graph algorithms: Dijkstra, A*, Bellman-Ford -- pathfinding over [[data structures]] like [[graphs]]
- dynamic programming: breaking problems into overlapping subproblems, memoization
- greedy: locally optimal choices at each step
- divide and conquer: recursively splitting the problem, then merging solutions

## relation to machines

An algorithm is independent of hardware. Its abstract form lives in [[automata]] theory; its concrete execution depends on [[compilers]] and [[operating systems]].