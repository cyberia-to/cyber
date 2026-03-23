---
tags: aip, cyb, cybernet
crystal-type: entity
crystal-domain: cyber
stake: 20308505166547020
diffusion: 0.00011233815923477823
springs: 0.0021446219400589202
heat: 0.001484988449252365
focus: 0.0009965533514855673
gravity: 0
density: 2.14
---
the [[cybernet]] app in [[cyb]]

gamification metaphor: magic school

## terminology

| protocol term | cyberver term |
|---------------|--------------|
| network | verse |
| root | root |
| subnetwork | faculty |
| uid | ticket |
| validator | medium |
| delegate | class of medium |
| miner | mage |
| delegator | warrior |

O (oxygen) [[token]] is for the cyber verse

## create verse

explore epochs

explore verses (ListDeployedContracts)

explore verse (listSubnetworks)

explore faculty (listSubnetwork)

deploy verse (InstantiateCybernet)
- TODO burn 1 TH
- TODO pricing for verses
- max mentors
- max learners
- [[particle]] with description

deploy faculty (RegisterNetwork)
- [[particle]] description
- DissolveNetwork: netuid (u16)

## become mage

mining is hard — good visualization map of steps

personal dashboard of mage

Register
- netuid: u16
- block_number: u64
- nonce: u64
- work: Vec
- hotkey: String
- coldkey: String

BurnedRegister
- netuid: u16
- hotkey: String

ServeAxon
- netuid: u16
- version: u32
- ip: u128
- port: u16
- ip_type: u8
- protocol: u8
- placeholder1: u8
- placeholder2: u8

ServePrometheus
- netuid: u16
- version: u32
- ip: u128
- port: u16
- ip_type: u8

RootRegister
- hotkey: String

BecomeDelegate
- hotkey: String
- take: u16

my verses and faculties with performance

set weights for root and faculties

SetWeights
- netuid: u16
- dests: Vec
- weights: Vec
- version_key: u64

## become warrior

explore mediums by their apr

check consolidated stats on the particular medium

check their answers

stake on them

AddStake
- hotkey: String
- amount_staked: u64

RemoveStake
- hotkey: String
- amount_unstaked: u64