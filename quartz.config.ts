// This file is copied into the Quartz directory during build
// See .github/workflows/publish.yml

import { QuartzConfig } from "./quartz/cfg"
import * as Plugin from "./quartz/plugins"

const config: QuartzConfig = {
  configuration: {
    pageTitle: "Cyber",
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
        header: "Play",
        body: "Play",
        code: "JetBrains Mono",
      },
      colors: {
        lightMode: {
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
        keepBackground: false,
      }),
      Plugin.ObsidianFlavoredMarkdown({ enableInHtmlEmbed: false }),
      Plugin.GitHubFlavoredMarkdown(),
      Plugin.TableOfContents(),
      Plugin.CrawlLinks({ markdownLinkResolution: "shortest" }),
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
      Plugin.Favicon(),
      Plugin.NotFoundPage(),
    ],
  },
}

export default config
