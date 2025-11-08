## species to research
- {{query (and (page-tags [[species]]) (not (page-tags [[class]])) (and (page-tags [[research]])))}}
  query-sort-by:: page
  query-sort-desc:: false
- ## genus to research
- {{query (and (page-tags [[genus]]) (not (page-tags [[class]])) (and (page-tags [[research]])))}}
  query-properties:: [:page :alias :tags]