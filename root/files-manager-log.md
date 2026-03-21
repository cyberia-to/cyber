---
tags: cyber
crystal-type: entity
crystal-domain: cyber
source: files-manager
icon: 
stake: 14150048611835708
diffusion: 0.00010722364868599256
springs: 0.00007019991600688145
heat: 0.00003419142694206788
focus: 0.00008151008453347325
gravity: 0
density: 0
---
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