---
tags: cyber, cyb, cybernomics
alias: cyb-boot, boot.dat
icon: "🚀"
crystal-type: entity
crystal-domain: cyb
---
a ~3MB installer for [[cyb]]: `boot.dat` carries a mnemonic and a referrer, fetched by content address (CID) rather than through an app store. Running it is the first sync — and the first sync is also the moment a [[referral]] binds: the referrer baked into `boot.dat` becomes a cyberlink, unprompted and automatic

this is the mechanics behind visibility as growth: every install already carries who sent it, so distribution needs no separate tracking layer — the referral graph is a byproduct of onboarding, not bolted onto it after the fact

wiring: `cyb-boot-server` serves patched installer artifacts (encrypted wallet data baked in per referrer) behind [[cybernode]]'s proxy, auto-deployed on push via a GitHub webhook

see [[referral]], [[cyb]], [[cybernode]]

discover all [[concepts]]
