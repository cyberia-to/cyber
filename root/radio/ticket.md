---
alias: BlobTicket, blob ticket, radio ticket
tags: cyber
crystal-type: entity
crystal-domain: cyber
---

# ticket

a serialized token containing everything needed to fetch a [[radio/blob]] or join a [[radio/docs]] replica

## contents

EndpointAddr (who to connect to) + [[Hemera]] hash (what to fetch) + BlobFormat (raw or [[radio/hash-seq]]). compact wire format using postcard serialization

## zero-coordination transfer

share a ticket and the recipient can immediately connect and download. no prior relationship, no [[radio/discovery]] step, no out-of-band coordination required

## use cases

share a single file with a blob ticket in raw format. share a collection with a blob ticket in [[radio/hash-seq]] format. share a document with a doc ticket carrying namespace and capability information

## sharing

tickets serialize to a string that works anywhere text works — paste in a chat, embed in a QR code, publish as a [[cyberlink]], store in a [[radio/docs]] entry

## role in cyber

tickets are how [[particles]] spread outside the [[cybergraph]]. anyone with the ticket can fetch the content directly from the providing [[neuron]] via [[radio]]. the ticket encodes the full retrieval path so no global registry or lookup service is needed