// This file is copied into the Quartz directory during build
// See .github/workflows/publish.yml

import { PageLayout, SharedLayout } from "./quartz/cfg"
import * as Component from "./quartz/components"

// Components shared across all pages
export const sharedPageComponents: SharedLayout = {
  head: Component.Head(),
  header: [],
  afterBody: [],
  footer: Component.Footer({
    links: {
      GitHub: "https://github.com/cybercongress/cyber",
      "cyb.ai": "https://cyb.ai",
    },
  }),
}

// Shared favorites list
const favorites = [
  "cyber",
  "aicosystem",
  "ask",
  "learn",
  "truth",
  "brain",
  "the product",
  "collective intelligence",
  "superintelligence",
  "concepts",
  "space pussy",
  "knowledge theory",
  "bostrom",
  "neuron",
  "particle",
  "tokens",
  "cyberlink",
  "cybergraph",
  "truth machine",
  "cybernet",
  "soft3",
  "projects",
  "todo",
  "energy reform",
  "aips",
  "cyb",
  "about this metagraph",
]

// Shared left sidebar components
const sharedLeftSidebar = [
  Component.PageTitle(),
  Component.MobileOnly(Component.Spacer()),
  Component.DesktopOnly(Component.Flex({
    components: [
      { Component: Component.Search(), grow: true },
      { Component: Component.Darkmode(), shrink: false },
      { Component: Component.SidebarToggle(), shrink: false },
    ],
    direction: "row",
    gap: "0.75rem",
  })),
  Component.MobileOnly(Component.Search()),
  Component.MobileOnly(Component.Darkmode()),
  Component.DesktopOnly(Component.Favorites({ title: "Favorites", favorites })),
  Component.DesktopOnly(Component.Explorer()),
]

// Components for pages that display a single piece of content
export const defaultContentPageLayout: PageLayout = {
  beforeBody: [
    Component.Breadcrumbs(),
    Component.ArticleTitle(),
    Component.ContentMeta(),
    Component.TagList(),
  ],
  left: sharedLeftSidebar,
  right: [
    Component.Graph(),
    Component.TableOfContents(),
    Component.Backlinks(),
  ],
}

// Components for pages that display lists of pages
export const defaultListPageLayout: PageLayout = {
  beforeBody: [Component.Breadcrumbs(), Component.ArticleTitle(), Component.ContentMeta()],
  left: sharedLeftSidebar,
  right: [
    Component.Graph(),
    Component.Backlinks(),
  ],
}
