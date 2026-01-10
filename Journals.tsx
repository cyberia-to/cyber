import { QuartzComponent, QuartzComponentConstructor, QuartzComponentProps } from "./types"
import { resolveRelative } from "../util/path"
import { classNames } from "../util/lang"

// @ts-ignore
import style from "./styles/journals.scss"
// @ts-ignore
import script from "./scripts/journals.inline"

interface Options {
  title: string
  limit: number
  defaultCollapsed: boolean
}

const defaultOptions: Options = {
  title: "Journals",
  limit: 5,
  defaultCollapsed: false,
}

export default ((userOpts?: Partial<Options>) => {
  const opts = { ...defaultOptions, ...userOpts }

  const Journals: QuartzComponent = ({
    allFiles,
    fileData,
    displayClass,
  }: QuartzComponentProps) => {
    // Filter journal entries
    const journals = allFiles
      .filter((f) => f.slug?.startsWith("journals/") && f.slug !== "journals/index")
      .sort((a, b) => {
        const dateA = a.frontmatter?.date || a.slug || ""
        const dateB = b.frontmatter?.date || b.slug || ""
        return dateB.toString().localeCompare(dateA.toString())
      })
      .slice(0, opts.limit)

    // Get all slugs for proper folder-page detection in resolveRelative
    const allSlugs = allFiles.map((f) => f.slug!)
    const journalsLink = resolveRelative(fileData.slug!, "journals", allSlugs) + "/"

    return (
      <div class={classNames(displayClass, "journals")}>
        <button
          type="button"
          class="journals-toggle"
          aria-expanded={!opts.defaultCollapsed}
        >
          <h2>
            <a href={journalsLink} class="internal">
              {opts.title}
            </a>
          </h2>
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
        <div class="journals-content">
          {journals.length > 0 ? (
            <ul>
              {journals.map((journal) => {
                const title = journal.frontmatter?.title ?? journal.slug
                const link = resolveRelative(fileData.slug!, journal.slug!, allSlugs)
                return (
                  <li>
                    <a href={link} class="internal" data-for={journal.slug}>
                      {title}
                    </a>
                  </li>
                )
              })}
            </ul>
          ) : (
            <p class="journal-empty">No journal entries yet.</p>
          )}
          {journals.length > 0 && (
            <p class="journal-see-more">
              <a href={journalsLink} class="internal">
                See all journals →
              </a>
            </p>
          )}
        </div>
      </div>
    )
  }

  Journals.css = style
  Journals.afterDOMLoaded = script

  return Journals
}) satisfies QuartzComponentConstructor
