---
icon: 🔒
alias: bostrom security, bostrom/security
tags: bostrom, infrastructure, security
crystal-type: entity
crystal-domain: cyber
stake: 35270161707214084
---
# Security

- Back to [[bostrom infrastructure]]

## Blockchain Security

- ### Consensus
	- Bostrom uses CometBFT (Tendermint) consensus
	- Byzantine fault tolerant up to 1/3 malicious validators
	- GPU-based proof-of-work component for ranking
- ### Validators
	- 100 active validators secure the network
	- Delegated Proof-of-Stake (DPoS) model
	- Slashing for double-signing and downtime
- ### Smart Contracts
	- CosmWasm-based smart contracts
	- Permissioned deployment (governance approval required)
	- Code review recommended before interaction

## Infrastructure Security

- ### Network Protection
	- All public endpoints use TLS 1.3 encryption
	- DDoS protection at the reverse proxy layer
	- Rate limiting to prevent abuse
	- Regular security updates and patching
- ### Access Control
	- Infrastructure servers use key-based SSH authentication
	- Two-factor authentication required for operators
	- Principle of least privilege for service accounts
- ### Monitoring
	- 24/7 automated monitoring with alerting
	- Intrusion detection systems
	- Regular log analysis

## User Security Best Practices

- ### Wallet Security
	- ✅ Use hardware wallets (Ledger) when possible
	- ✅ Keplr through Ledger is the recommended setup
	- ✅ Never share your seed phrase
	- ✅ Verify transaction details before signing
	- ⚠️ Be cautious of phishing sites claiming to be cyb.ai
- ### Verifying Authenticity
	- Official domain: `cyb.ai` (not cyb.io, cyb.net, etc.)
	- Check SSL certificate: Should be valid and issued to the correct domain
	- IPFS version: cyb.ai is also available via IPFS for censorship resistance
- ### IBC Transfers
	- Always double-check recipient addresses
	- Use small test transfers first for new addresses
	- Be aware of timeout periods (usually 10 minutes)

## Responsible Disclosure

- If you discover a security vulnerability:
	- DO NOT disclose publicly before it's fixed
	- Contact the team via:
		- Telegram: DM to @groovybear (mastercyb)
		- Email: security concerns to the core team
	- Provide detailed reproduction steps
	- Allow reasonable time for fixes

## Incident Response

- In case of security incidents:
	- Infrastructure team is alerted via monitoring
	- Critical issues trigger immediate response
	- Post-mortems are published for significant incidents

## Audits

| Component | Audit Status |
|-----------|--------------|
| go-cyber | Community reviewed, no formal audit |
| CosmWasm contracts | Per-contract basis |
| Infrastructure | Regular security reviews |

## Known Risks

- ### Smart Contract Risk
	- Contracts may contain bugs; verify before interacting
- ### Centralization Risk
	- Validator set concentration — stake with diverse validators
- ### IBC Risk
	- Cross-chain transfers depend on relayer availability
	- Tokens can be stuck if channels expire (recoverable via governance)
- ### Regulatory Risk
	- Cryptocurrency regulations vary by jurisdiction

## Security Updates

- Follow these channels for security announcements:
	- Telegram: https://t.me/bostrom_news
	- Twitter/X: https://x.com/cyber_devs
	- GitHub: Watch the go-cyber repository

## Related

- [[bostrom infrastructure]]
- [[go-cyber]]
