import { QuartzComponent, QuartzComponentConstructor, QuartzComponentProps } from "./types"
import { resolveRelative, simplifySlug } from "../util/path"
import { classNames } from "../util/lang"

// @ts-ignore
import style from "./styles/favorites.scss"
// @ts-ignore
import script from "./scripts/favorites.inline"

interface FavoritesOptions {
  title: string
  favorites: string[]
  defaultCollapsed: boolean
}

const defaultOptions: FavoritesOptions = {
  title: "Favorites",
  favorites: [],
  defaultCollapsed: false,
}

export default ((opts?: Partial<FavoritesOptions>) => {
  const options: FavoritesOptions = { ...defaultOptions, ...opts }

  const Favorites: QuartzComponent = ({
    fileData,
    allFiles,
    displayClass,
  }: QuartzComponentProps) => {
    // Find files that match the favorites list
    const favoriteFiles = options.favorites
      .map((fav) => {
        const slug = fav.toLowerCase().replace(/ /g, "-")
        return allFiles.find(
          (file) =>
            simplifySlug(file.slug!) === slug ||
            file.frontmatter?.title?.toLowerCase() === fav.toLowerCase(),
        )
      })
      .filter((f) => f !== undefined)

    if (favoriteFiles.length === 0) {
      return null
    }

    // Get all slugs for proper folder-page detection in resolveRelative
    const allSlugs = allFiles.map((f) => f.slug!)

    return (
      <div class={classNames(displayClass, "favorites")}>
        <button
          type="button"
          class="favorites-toggle"
          aria-expanded={!options.defaultCollapsed}
        >
          <h2>{options.title}</h2>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="14"
            height="14"
            viewBox="5 8 14 8"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="fold"
          >
            <polyline points="6 9 12 15 18 9"></polyline>
          </svg>
        </button>
        <div class="favorites-content">
          <ul>
            {favoriteFiles.map((f) => {
              const icon = f!.frontmatter?.icon || ""
              let title = f!.frontmatter?.title || simplifySlug(f!.slug!)
              if (icon && title.startsWith(icon)) {
                title = title.slice(icon.length).trim()
              }
              const href = resolveRelative(fileData.slug!, f!.slug!, allSlugs)
              return (
                <li>
                  <a href={href} class="internal" data-for={f!.slug}>
                    {icon && <span class="favorite-icon">{icon}</span>}
                    {title}
                  </a>
                </li>
              )
            })}
          </ul>
        </div>
      </div>
    )
  }

  Favorites.css = style
  Favorites.afterDOMLoaded = script

  return Favorites
}) satisfies QuartzComponentConstructor
