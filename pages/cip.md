alias:: cyber improvement proposal, cyber improvement proposals, list of cips, cips
tags:: cyber
- ## what is cip?
	- cyber improvement proposal
	- the process of implementing consensus wide changes in
		- [[go-cyber]]
		- [[cw-cyber]]
- ## states
	- draft: open for discussion
	- accepted: finalized for implementation
	- rejected: discounted after discussion
	- testing: deployed to [[spacepussy]]
	- implemented: deployed to [[bostrom]]
- ## implemented
	- {{query (and (page-tags [[cip]]) (page-property :status "implemented"))}}
	  query-properties:: [:page]
- ## tested
	- {{query (and (page-tags [[cip]]) (page-property :status "tested"))}}
	  query-properties:: [:page]
- ## accepted
	- {{query (and (page-tags [[cip]]) (page-property :status "accepted"))}}
	  query-properties:: [:page]
- ## draft
	- {{query (and (page-tags [[cip]]) (page-property :status "draft"))}}
	  query-properties:: [:page]
	  query-sort-by:: page
	  query-sort-desc:: false
- ## rejected
	- {{query (and (page-tags [[cip]]) (page-property :status "rejected"))}}
	  query-properties:: [:page]
