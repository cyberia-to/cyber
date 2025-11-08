source:: [[files-manager]]
icon:: 

- query-sort-by:: date
                                                      query-sort-desc:: true
                                                      #+BEGIN_QUERY
                                                      {:title "All files operate log"
                                                      :query [:find (pull ?b [*])
                                                              :in $ ?current-page
                                                              :where
                                                              [?p :block/name ?current-page]
                                                              [?b :block/page ?p]
                                                              [?b :block/content ?content]
                                                              [(!= ?content "")]
                                                              ]
                                                      :inputs ["files-manager"]
                                                      :limit 10 ; 每页限制返回10条记录
                                                      :offset 0 ; 从第一条记录开始    
                                                      :table-view? false
                                                      }
                                                      #+END_QUERY