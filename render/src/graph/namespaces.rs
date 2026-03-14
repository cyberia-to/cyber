use super::PageStore;

pub fn build_namespace_tree(store: &mut PageStore) {
    let entries: Vec<(String, Option<String>)> = store
        .pages
        .iter()
        .map(|(id, page)| (id.clone(), page.namespace.clone()))
        .collect();

    for (page_id, namespace) in entries {
        if let Some(ns) = namespace {
            // Register under direct namespace
            store
                .namespace_tree
                .entry(ns.clone())
                .or_default()
                .push(page_id.clone());

            // Also register under all ancestor namespaces so the root
            // page of a deep tree (e.g., "trident") sees all descendants.
            let mut ancestor = ns.as_str();
            while let Some(pos) = ancestor.rfind('/') {
                ancestor = &ancestor[..pos];
                store
                    .namespace_tree
                    .entry(ancestor.to_string())
                    .or_default()
                    .push(page_id.clone());
            }
        }
    }
}
