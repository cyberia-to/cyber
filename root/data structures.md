---
tags: computer science
crystal-type: entity
crystal-domain: computer science
stake: 5177611082825399
diffusion: 0.00026868947278207197
springs: 0.00011392275792056936
heat: 0.0001767654165342816
focus: 0.00020387464707406048
gravity: 5
density: 3.78
---

Ways of organizing, storing, and accessing data so that [[algorithms]] can operate efficiently.

## linear

array: contiguous block of memory, O(1) random access

linked list: nodes connected by pointers, O(1) insertion/deletion

stack: last-in first-out (LIFO)

queue: first-in first-out (FIFO)

## hierarchical

tree: nodes with parent-child relationships, binary trees, B-trees

heap: tree satisfying the heap property, used in priority queues

trie: prefix tree for fast string lookup

## associative

hash table: key-value storage with O(1) average lookup via hashing

[[graphs]]: nodes and edges representing relationships, directed/undirected, weighted

## graph

A graph is a set of vertices connected by edges. Foundation of [[knowledge graphs]], [[neural networks]], and network science. Represented as adjacency lists or adjacency matrices.

## choosing

The right structure depends on access patterns: random access favors arrays, frequent insertions favor linked lists, key lookups favor hash tables, hierarchical queries favor trees. [[databases]] formalize these tradeoffs at scale.