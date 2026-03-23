---
tags: cyberia
crystal-type: entity
crystal-domain: cyberia
stake: 4846458775021527
diffusion: 0.00011233815923477823
springs: 0.00008246788381555257
heat: 0.000037649645370807764
focus: 0.00008843937383621902
gravity: 0
density: 0
---
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