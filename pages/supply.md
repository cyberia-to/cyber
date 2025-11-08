- {{query (property :supply "yes")}}
  query-table:: true
  query-properties:: [:page :alias :margin]
-
- ## ironwoods
- {{query (and (property :supply "next-month") (property :market "ironwoods"))}}
  query-table:: true
  query-properties:: [:page :alias :wood-density]
  query-sort-by:: wood-density
  query-sort-desc:: true
- ## hardwoods
- {{query (and (property :supply "next-month") (property :project "hardwoods"))}}
  query-table:: true
  query-properties:: [:page :alias :wood-density]
  query-sort-by:: wood-density
  query-sort-desc:: true
- ## resins
- {{query (and (property :supply "next-month") (property :project "resins"))}}
  query-table:: true
  query-properties:: [:page :alias :wood-density]
- ## rhizomes
- {{query (and (property :supply "next-month") (property :project "rhizomes"))}}
  query-table:: true
  query-properties:: [:page :project]
- {{query (property :supply "next-month")}}
  query-table:: true
  query-properties:: [:page :project]
  query-sort-by:: page
  query-sort-desc:: false
-
- {{query (and (page-property :supply "next-month") (page-property :project "edible-oil"))}}
  query-properties:: [:page :alias :wood-density]
-
-
-
- {{query (property :state "supply")}}
  query-table:: true
  query-properties:: [:page]