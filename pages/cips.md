alias:: cyber improvement proposals, list of cips
- ## what is [[cip]]?
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