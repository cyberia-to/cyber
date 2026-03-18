---
tags: cyber
crystal-type: entity
crystal-domain: cyber
query-table: "true"
stake: 14203748986074172
---
#+BEGIN_QUERY
{:title "TODO tasks"
 :query [:find (pull ?b [*])
         :where
         (task ?b #{"TODO"})]}
#+END_QUERY

#+BEGIN_QUERY
{:title "All blocks with tag project"
 :query [:find (pull ?b [*])
         :where
         [?p :block/name "species"]
         [?b :block/refs ?p]]}
#+END_QUERY

#+BEGIN_QUERY
{:title "All tasks"
 :query [:find (pull ?b [*])
         :where
         [?b :block/marker _]]}
#+END_QUERY
