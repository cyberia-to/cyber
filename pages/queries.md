query-table:: true
#+BEGIN_QUERY
{:title "TODO tasks"
 :query [:find (pull ?b [*])
         :where
         (task ?b #{"TODO"})]}
#+END_QUERY

- query-table:: false
  collapsed:: true
  #+BEGIN_QUERY
  {:title "All blocks with tag project"
   :query [:find (pull ?b [*])
           :where
           [?p :block/name "species"]
           [?b :block/refs ?p]]}
  #+END_QUERY
- query-table:: true
  #+BEGIN_QUERY
  {:title "All tasks"
   :query [:find (pull ?b [*])
           :where
           [?b :block/marker _]]}
  #+END_QUERY