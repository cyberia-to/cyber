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

// Components for pages that display a single piece of content
export const defaultContentPageLayout: PageLayout = {
  beforeBody: [
    Component.ConditionalRender({
      component: Component.Breadcrumbs(),
      condition: (page) => page.fileData.slug !== "index",
    }),
    Component.ArticleTitle(),
    Component.ContentMeta(),
    Component.TagList(),
  ],
  left: [
    Component.PageTitle(),
    Component.MobileOnly(Component.Spacer()),
    Component.Flex({
      components: [
        { Component: Component.Search(), grow: true },
        { Component: Component.ReaderMode() },
      ],
    }),
    Component.Explorer({
      sortFn: (a, b) => {
        // Favorites from Logseq config + journals at top
        // Favorites, journals, then pages folder
        const priority = ["favorites", "journals", "pages"]
        const aIdx = priority.indexOf(a.slugSegment)
        const bIdx = priority.indexOf(b.slugSegment)

        // Both are priority items - sort by priority order
        if (aIdx !== -1 && bIdx !== -1) return aIdx - bIdx
        // Only a is priority - a comes first
        if (aIdx !== -1) return -1
        // Only b is priority - b comes first
        if (bIdx !== -1) return 1

        // Neither is priority - folders first, then alphabetical
        if (a.isFolder && !b.isFolder) return -1
        if (!a.isFolder && b.isFolder) return 1
        return a.displayName.localeCompare(b.displayName, undefined, {
          numeric: true,
          sensitivity: "base",
        })
      },
    }),
  ],
  right: [
    Component.Graph(),
    Component.DesktopOnly(Component.TableOfContents()),
    Component.Backlinks(),
  ],
}

// Components for pages that display lists of pages
export const defaultListPageLayout: PageLayout = {
  beforeBody: [Component.Breadcrumbs(), Component.ArticleTitle(), Component.ContentMeta()],
  left: [
    Component.PageTitle(),
    Component.MobileOnly(Component.Spacer()),
    Component.Flex({
      components: [
        { Component: Component.Search(), grow: true },
      ],
    }),
    Component.Explorer({
      sortFn: (a, b) => {
        // Favorites from Logseq config + journals at top
        // Favorites, journals, then pages folder
        const priority = ["favorites", "journals", "pages"]
        const aIdx = priority.indexOf(a.slugSegment)
        const bIdx = priority.indexOf(b.slugSegment)

        // Both are priority items - sort by priority order
        if (aIdx !== -1 && bIdx !== -1) return aIdx - bIdx
        // Only a is priority - a comes first
        if (aIdx !== -1) return -1
        // Only b is priority - b comes first
        if (bIdx !== -1) return 1

        // Neither is priority - folders first, then alphabetical
        if (a.isFolder && !b.isFolder) return -1
        if (!a.isFolder && b.isFolder) return 1
        return a.displayName.localeCompare(b.displayName, undefined, {
          numeric: true,
          sensitivity: "base",
        })
      },
    }),
  ],
  right: [
    Component.Graph(),
    Component.Backlinks(),
  ],
}
