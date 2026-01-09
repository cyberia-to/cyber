import { QuartzComponent, QuartzComponentConstructor, QuartzComponentProps } from "./types"
import { pathToRoot, simplifySlug } from "../util/path"
import { classNames } from "../util/lang"

interface FavoritesOptions {
  title: string
  favorites: string[]
}

const defaultOptions: FavoritesOptions = {
  title: "Favorites",
  favorites: [],
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
            file.frontmatter?.title?.toLowerCase() === fav.toLowerCase()
        )
      })
      .filter((f) => f !== undefined)

    if (favoriteFiles.length === 0) {
      return null
    }

    // Calculate path to root (accounting for folder/index.html structure)
    let baseDir = pathToRoot(fileData.slug!)
    if (baseDir === ".") {
      baseDir = ".."
    } else {
      baseDir = "../" + baseDir
    }

    return (
      <div class={classNames(displayClass, "favorites")}>
        <h3>{options.title}</h3>
        <ul>
          {favoriteFiles.map((f) => {
            const icon = f!.frontmatter?.icon || ""
            let title = f!.frontmatter?.title || simplifySlug(f!.slug!)
            // Strip leading emoji from title if icon is present (avoid double icons)
            if (icon && title.startsWith(icon)) {
              title = title.slice(icon.length).trim()
            }
            // Use path from root to target slug
            const href = baseDir + "/" + f!.slug!
            return (
              <li>
                <a href={href} class="internal">
                  {icon && <span class="favorite-icon">{icon}</span>}
                  {title}
                </a>
              </li>
            )
          })}
        </ul>
      </div>
    )
  }

  Favorites.css = `
.favorites {
  margin-bottom: 1rem;
}

.favorites h3 {
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--gray);
  margin-bottom: 0.5rem;
}

.favorites ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.favorites li {
  margin: 0.3rem 0;
}

.favorites a {
  color: var(--dark);
  text-decoration: none;
  font-size: 0.9rem;
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.favorites a:hover {
  color: var(--secondary);
}

.favorite-icon {
  font-size: 1rem;
  line-height: 1;
}
`

  return Favorites
}) satisfies QuartzComponentConstructor
