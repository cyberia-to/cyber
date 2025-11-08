icon:: 🦚

- ## selected genus for [[citadel genesis]]
- {{query (and (page-tags [[genus]]) (not (page-tags [[class]])) (not (page-tags [[research]])) (not (page-tags [[prohibited]])))}}
  query-sort-by:: page
  query-sort-desc:: false
  query-properties:: [:page :alias]
  background-color:: blue
- ## genus to research
- {{query (and (page-tags [[genus]]) (not (page-tags [[class]])) (and (page-tags [[research]])))}}
  query-properties:: [:page :alias :tags :scalable]
  query-sort-by:: scalable
  query-sort-desc:: true
- ## prohibited genus
- {{query (and (page-tags [[genus]]) (not (page-tags [[class]])) (and (page-tags [[prohibited]] )))}}
  query-properties:: [:page :tags]