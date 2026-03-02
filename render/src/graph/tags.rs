use super::PageStore;

pub fn build_tag_index(store: &mut PageStore) {
    let entries: Vec<(String, Vec<String>)> = store
        .pages
        .iter()
        .map(|(id, page)| (id.clone(), page.meta.tags.clone()))
        .collect();

    for (page_id, tags) in entries {
        for tag in tags {
            let tag_lower = tag.to_lowercase();
            store
                .tag_index
                .entry(tag_lower)
                .or_default()
                .push(page_id.clone());
        }
    }
}
