// This file is copied into the Quartz directory during build
// See .github/workflows/publish.yml

import { QuartzConfig } from "./quartz/cfg"
import * as Plugin from "./quartz/plugins"

const config: QuartzConfig = {
  configuration: {
    pageTitle: "cyber docs",
    pageTitleSuffix: "",
    enableSPA: true,
    enablePopovers: true,
    analytics: {
      provider: "plausible",
      host: "https://metrics.cyb.ai",
    },
    locale: "en-US",
    baseUrl: "docs.cyb.ai",
    ignorePatterns: ["private", "templates", ".obsidian"],
    defaultDateType: "modified",
    theme: {
      fontOrigin: "googleFonts",
      cdnCaching: true,
      typography: {
        header: "Inter",
        body: "Inter",
        code: "JetBrains Mono",
      },
      colors: {
        lightMode: {
          light: "#1a1a1a",
          lightgray: "#2a2a2a",
          gray: "#4a4a4a",
          darkgray: "#b0b0b0",
          dark: "#e0e0e0",
          secondary: "#4cc38a",
          tertiary: "#3cb179",
          highlight: "rgba(76, 195, 138, 0.15)",
          textHighlight: "rgba(76, 195, 138, 0.3)",
        },
        darkMode: {
          light: "#000000",
          lightgray: "#141414",
          gray: "#3a3a3a",
          darkgray: "#a0a0a0",
          dark: "#e0e0e0",
          secondary: "#4cc38a",
          tertiary: "#3cb179",
          highlight: "rgba(76, 195, 138, 0.15)",
          textHighlight: "rgba(76, 195, 138, 0.3)",
        },
      },
    },
  },
  plugins: {
    transformers: [
      Plugin.FrontMatter(),
      Plugin.CreatedModifiedDate({
        priority: ["frontmatter", "git", "filesystem"],
      }),
      Plugin.SyntaxHighlighting({
        theme: {
          light: "github-dark",
          dark: "github-dark",
        },
      }),
      Plugin.ObsidianFlavoredMarkdown({
        enableInHtmlEmbed: false,
        enableCheckbox: true,  // Enable interactive TODO checkboxes
        parseBlockReferences: true,  // Enable ^block-id references
        mermaid: true,  // Enable mermaid diagrams
        callouts: true,  // Enable callout blocks
        wikilinks: true,  // Enable [[wikilinks]]
        highlight: true,  // Enable ==highlights==
        parseTags: true,  // Enable #tags
        enableYouTubeEmbed: true,  // Embed YouTube videos
        enableVideoEmbed: true,  // Embed video files
      }),
      Plugin.GitHubFlavoredMarkdown({
        enableSmartyPants: true,  // Smart quotes and dashes
        linkHeadings: true,  // Add anchor links to headings
      }),
      Plugin.TableOfContents(),
      Plugin.CrawlLinks({ markdownLinkResolution: "relative" }),
      Plugin.Description(),
      Plugin.Latex({ renderEngine: "katex" }),
    ],
    filters: [Plugin.RemoveDrafts()],
    emitters: [
      Plugin.AliasRedirects(),
      Plugin.ComponentResources(),
      Plugin.ContentPage(),
      Plugin.FolderPage(),
      Plugin.TagPage(),
      Plugin.ContentIndex({
        enableSiteMap: true,
        enableRSS: true,
      }),
      Plugin.Assets(),
      Plugin.Static(),
      Plugin.NotFoundPage(),
    ],
  },
}

export default config
