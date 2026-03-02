use super::PageStore;

pub fn build_namespace_tree(store: &mut PageStore) {
    let entries: Vec<(String, Option<String>)> = store
        .pages
        .iter()
        .map(|(id, page)| (id.clone(), page.namespace.clone()))
        .collect();

    for (page_id, namespace) in entries {
        if let Some(ns) = namespace {
            store
                .namespace_tree
                .entry(ns)
                .or_default()
                .push(page_id);
        }
    }
}
