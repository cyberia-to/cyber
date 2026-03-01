use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct SiteConfig {
    pub site: SiteSection,
    pub nav: NavSection,
    pub build: BuildSection,
    pub content: ContentSection,
    pub urls: UrlsSection,
    pub feeds: FeedsSection,
    pub search: SearchSection,
    pub analytics: AnalyticsSection,
    pub graph: GraphSection,
    pub style: StyleSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SiteSection {
    pub title: String,
    pub description: String,
    pub base_url: String,
    pub language: String,
    pub root_page: Option<String>,
    pub favicon: Option<String>,
}

impl Default for SiteSection {
    fn default() -> Self {
        Self {
            title: "My Knowledge Base".to_string(),
            description: String::new(),
            base_url: "http://localhost:8080".to_string(),
            language: "en".to_string(),
            root_page: None,
            favicon: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct NavSection {
    pub menu: Vec<MenuItem>,
    /// When set, auto-generate menu from pages that have this tag (e.g. "menu").
    /// Overrides the static `menu` list above.
    pub menu_tag: Option<String>,
    pub sidebar: SidebarSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub children: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SidebarSection {
    pub show_namespaces: bool,
    pub show_recent: bool,
    pub recent_count: usize,
    pub show_tags: bool,
}

impl Default for SidebarSection {
    fn default() -> Self {
        Self {
            show_namespaces: true,
            show_recent: true,
            recent_count: 10,
            show_tags: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildSection {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub template_dir: Option<PathBuf>,
    pub static_dir: Option<PathBuf>,
}

impl Default for BuildSection {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("."),
            output_dir: PathBuf::from("build"),
            template_dir: None,
            static_dir: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContentSection {
    pub public_only: bool,
    pub exclude_patterns: Vec<String>,
    pub include_journals: bool,
    pub default_public: bool,
}

impl Default for ContentSection {
    fn default() -> Self {
        Self {
            public_only: true,
            exclude_patterns: vec![
                "logseq/*".to_string(),
                "draws/*".to_string(),
                ".git/*".to_string(),
                "build/*".to_string(),
                "target/*".to_string(),
                ".DS_Store".to_string(),
            ],
            include_journals: false,
            default_public: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UrlsSection {
    pub style: String,
    pub slugify: bool,
}

impl Default for UrlsSection {
    fn default() -> Self {
        Self {
            style: "pretty".to_string(),
            slugify: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FeedsSection {
    pub enabled: bool,
    pub title: Option<String>,
    pub items: usize,
}

impl Default for FeedsSection {
    fn default() -> Self {
        Self {
            enabled: true,
            title: None,
            items: 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchSection {
    pub enabled: bool,
    pub engine: String,
}

impl Default for SearchSection {
    fn default() -> Self {
        Self {
            enabled: true,
            engine: "json".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AnalyticsSection {
    pub plausible_domain: Option<String>,
    pub plausible_script: String,
    /// Raw HTML snippet to inject into <head>. When set, overrides plausible_script template.
    pub snippet: Option<String>,
}

impl Default for AnalyticsSection {
    fn default() -> Self {
        Self {
            plausible_domain: None,
            plausible_script: "https://plausible.io/js/script.js".to_string(),
            snippet: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GraphSection {
    pub enabled: bool,
    pub show_minimap: bool,
    pub minimap_depth: usize,
}

impl Default for GraphSection {
    fn default() -> Self {
        Self {
            enabled: true,
            show_minimap: true,
            minimap_depth: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StyleSection {
    pub primary_color: String,
    pub secondary_color: String,
    pub bg_color: String,
    pub text_color: String,
    pub surface_color: String,
    pub border_color: String,
    pub typography: TypographySection,
    pub code: CodeSection,
}

impl Default for StyleSection {
    fn default() -> Self {
        Self {
            primary_color: "#22c55e".to_string(),
            secondary_color: "#06b6d4".to_string(),
            bg_color: "#000000".to_string(),
            text_color: "#f0f0f0".to_string(),
            surface_color: "#111111".to_string(),
            border_color: "#222222".to_string(),
            typography: TypographySection::default(),
            code: CodeSection::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TypographySection {
    pub font_body: String,
    pub font_mono: String,
    pub font_size_base: String,
    pub line_height: String,
    pub max_width: String,
}

impl Default for TypographySection {
    fn default() -> Self {
        Self {
            font_body: "system-ui, -apple-system, 'Segoe UI', sans-serif".to_string(),
            font_mono: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace".to_string(),
            font_size_base: "1rem".to_string(),
            line_height: "1.7".to_string(),
            max_width: "48rem".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CodeSection {
    pub theme: String,
    pub show_line_numbers: bool,
}

impl Default for CodeSection {
    fn default() -> Self {
        Self {
            theme: "base16-ocean.dark".to_string(),
            show_line_numbers: false,
        }
    }
}

impl SiteConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: SiteConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(SiteConfig::default())
        }
    }

    pub fn with_overrides(mut self, base_url: Option<&str>, output_dir: Option<&Path>) -> Self {
        if let Some(url) = base_url {
            self.site.base_url = url.to_string();
        }
        if let Some(dir) = output_dir {
            self.build.output_dir = dir.to_path_buf();
        }
        self
    }
}
