use crate::parser::ParsedPage;
use std::path::PathBuf;

pub struct ValidationWarning {
    pub page_id: String,
    pub source_path: PathBuf,
    pub message: String,
}

const VALID_CRYSTAL_TYPES: &[&str] = &[
    "entity", "process", "property", "relation", "measure", "pattern", "article", "reference",
];

const VALID_CRYSTAL_DOMAINS: &[&str] = &[
    "cyber",
    "cyberia",
    "superhuman",
    "cybics",
    "biology",
    "chemistry",
    "economics",
    "physics",
    "computer science",
    "mathematics",
    "materials",
    "agriculture",
    "geography",
    "culture",
    "history",
    "governance",
    "energy",
];

pub fn validate_page(page: &ParsedPage) -> Vec<ValidationWarning> {
    let mut warnings = Vec::new();
    let props = &page.meta.properties;

    match props.get("crystal-type") {
        None => warnings.push(ValidationWarning {
            page_id: page.id.clone(),
            source_path: page.source_path.clone(),
            message: "missing crystal-type".to_string(),
        }),
        Some(v) if !VALID_CRYSTAL_TYPES.contains(&v.as_str()) => {
            warnings.push(ValidationWarning {
                page_id: page.id.clone(),
                source_path: page.source_path.clone(),
                message: format!("invalid crystal-type: '{}'", v),
            });
        }
        _ => {}
    }

    match props.get("crystal-domain") {
        None => warnings.push(ValidationWarning {
            page_id: page.id.clone(),
            source_path: page.source_path.clone(),
            message: "missing crystal-domain".to_string(),
        }),
        Some(v) if !VALID_CRYSTAL_DOMAINS.contains(&v.as_str()) => {
            warnings.push(ValidationWarning {
                page_id: page.id.clone(),
                source_path: page.source_path.clone(),
                message: format!("invalid crystal-domain: '{}'", v),
            });
        }
        _ => {}
    }

    warnings
}
