---
alias: cyber improvement proposal, cyber improvement proposals, list of cips, cips
tags: cyber
crystal-type: entity
crystal-domain: cyber
---
## what is cip?

- cyber improvement proposal
- the process of implementing consensus wide changes in
	- [[go-cyber]]
	- [[cw-cyber]]

## states

- draft: open for discussion
- accepted: finalized for implementation
- rejected: discounted after discussion
- testing: deployed to [[spacepussy]]
- implemented: deployed to [[bostrom]]

## implemented

- #+BEGIN_QUERY
	  {:query [:find (pull ?p [:block/name])
	   :where
	   [?p :block/tags ?t]
	   [?t :block/name "cip"]
	   [?p :block/properties ?props]
	   [(get ?props :status) ?s]
	   [(= ?s "implemented")]]
	   :result-transform (fn [r] (sort-by :block/name r))
	   :breadcrumb-show? false}
	  #+END_QUERY

## tested

- #+BEGIN_QUERY
	  {:query [:find (pull ?p [:block/name])
	   :where
	   [?p :block/tags ?t]
	   [?t :block/name "cip"]
	   [?p :block/properties ?props]
	   [(get ?props :status) ?s]
	   [(= ?s "tested")]]
	   :result-transform (fn [r] (sort-by :block/name r))
	   :breadcrumb-show? false}
	  #+END_QUERY

## accepted

- #+BEGIN_QUERY
	  {:query [:find (pull ?p [:block/name])
	   :where
	   [?p :block/tags ?t]
	   [?t :block/name "cip"]
	   [?p :block/properties ?props]
	   [(get ?props :status) ?s]
	   [(= ?s "accepted")]]
	   :result-transform (fn [r] (sort-by :block/name r))
	   :breadcrumb-show? false}
	  #+END_QUERY

## draft

- #+BEGIN_QUERY
	  {:query [:find (pull ?p [:block/name])
	   :where
	   [?p :block/tags ?t]
	   [?t :block/name "cip"]
	   [?p :block/properties ?props]
	   [(get ?props :status) ?s]
	   [(= ?s "draft")]]
	   :result-transform (fn [r] (sort-by :block/name r))
	   :breadcrumb-show? false}
	  #+END_QUERY

## rejected

- #+BEGIN_QUERY
	  {:query [:find (pull ?p [:block/name])
	   :where
	   [?p :block/tags ?t]
	   [?t :block/name "cip"]
	   [?p :block/properties ?props]
	   [(get ?props :status) ?s]
	   [(= ?s "rejected")]]
	   :result-transform (fn [r] (sort-by :block/name r))
	   :breadcrumb-show? false}
	  #+END_QUERY