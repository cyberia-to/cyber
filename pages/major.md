- {{query (and (page-tags [[major]]) (not (page-tags [[research]])))}}
  query-sort-by:: page
  query-sort-desc:: false
  query-properties:: [:page :alias :project :segment :tags]
-
- {{query (and (page-tags [[major]]) (page-tags [[research]]))}}
  query-sort-by:: page
  query-sort-desc:: false
  query-properties:: [:page :alias :project :market :supply]