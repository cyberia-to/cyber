- {{query (page-property :type "sector")}}
  query-sort-by:: page
  query-table:: true
  query-sort-desc:: false
  query-properties:: [:page :type :updated-at]
- query-sort-by:: block
  query-table:: true
  query-sort-desc:: false
  #+BEGIN_QUERY
  {:title "Blocks in 'edem/' pages"
   :query [:find (pull ?b [*])
           :where
           [?b :block/page ?p]
           [?p :page/name ?pname]
           [(clojure.string/includes? ?pname "edem/")]
           [(not= ?pname "edem/sectors")]]
   :inputs []
   :result-transform (fn [result] result)}
  #+END_QUERY/
-