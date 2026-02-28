---
tags: computer science
crystal-type: entity
crystal-domain: computer science
---

Systems for structured storage, retrieval, and manipulation of data. Persistent memory for [[computation]].

## models

relational: tables with rows and columns, SQL, joins, normalization. PostgreSQL, MySQL

graph: nodes and edges as first-class citizens, Cypher/SPARQL queries. Neo4j, [[knowledge graphs]]

key-value: simple pairs, high throughput. Redis, RocksDB

document: JSON/BSON documents, flexible schema. MongoDB, CouchDB

column-family: wide columns, optimized for reads at scale. Cassandra, HBase

## fundamentals

indexing: B-trees, hash indexes, inverted indexes for fast lookup

ACID: atomicity, consistency, isolation, durability -- transaction guarantees

CAP theorem: a distributed database can guarantee at most two of consistency, availability, and partition tolerance

query optimization: transforming queries into efficient execution plans using [[algorithms]]

## distributed databases

Sharding, replication, and [[consensus algorithms]] enable databases to scale across nodes. [[cyber]] uses distributed storage for the [[knowledge graph]].