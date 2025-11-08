- #+BEGIN_QUERY
  {
    :title "Unique Species with Amounts"
    :query [
      :find (pull ?block [*])
      :where
        [?page :block/name ?title]
        [(clojure.string/starts-with? ?title "edem/")]
        [?page :block/children ?block]
    ]
    :result-transform (fn [result]
      (->> result
           (map (fn [block]
                  {:species (get block :block/content)
                   :count 1}))
           (group-by :species)
           (map (fn [[species blocks]]
                  {:species species
                   :amount (count blocks)}))
           (sort-by :species)))
    :view (fn [rows]
      [:table
        [:thead
          [:tr [:th "Species"] [:th "Amount"]]]
        [:tbody
          (for [{:keys [species amount]} rows]
            [:tr
              [:td species]
              [:td amount]])]])
  }
  #+END_QUERY