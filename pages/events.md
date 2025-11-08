tags:: market, cv.land
focus:: true

- we are working on a permanent event platform for iconic events
- are you organizing events?
- let us know what would you do here => tg
- ## private camp rents
	- | period | price |
	  | working day | $30 |
	  | peak day | $50 |
	  | full moon | $70 |
	  | holyday moon | $100 |
	  | new moon | $200 |
	  | burn.city | $300 |
	- ## private camps
	- we have 7 private camps available for rent
	- {{query (and (page-tags [[camp]]) (not (page-property :type "public")))}}
	  query-properties:: [:page :tags :size :shape :type]
	  query-sort-by:: size
	  query-sort-desc:: true