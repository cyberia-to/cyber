import { QuartzComponent, QuartzComponentConstructor, QuartzComponentProps } from "./types"
import { pathToRoot, joinSegments } from "../util/path"
import { classNames } from "../util/lang"

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

  const Journals: QuartzComponent = ({ allFiles, fileData, displayClass }: QuartzComponentProps) => {
    // Filter journal entries
    const journals = allFiles
      .filter((f) => f.slug?.startsWith("journals/") && f.slug !== "journals/index")
      .sort((a, b) => {
        const dateA = a.frontmatter?.date || a.slug || ""
        const dateB = b.frontmatter?.date || b.slug || ""
        return dateB.toString().localeCompare(dateA.toString())
      })
      .slice(0, opts.limit)

    // Use root-relative path so Journals links work from any page
    const baseDir = pathToRoot(fileData.slug!)
    const journalsLink = joinSegments(baseDir, "journals/")

    return (
      <div class={classNames(displayClass, "journals")} data-collapsed={opts.defaultCollapsed}>
        <button type="button" class="journals-toggle" aria-expanded={!opts.defaultCollapsed}>
          <h3>
            <a href={journalsLink} class="internal" onClick="event.stopPropagation()">
              {opts.title}
            </a>
          </h3>
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
            <ul class="journal-list">
              {journals.map((journal) => {
                const title = journal.frontmatter?.title ?? journal.slug
                const link = joinSegments(baseDir, journal.slug!)
                return (
                  <li>
                    <a href={link} class="internal">
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

  Journals.css = `
    .journals {
      margin-top: 1rem;
    }
    .journals-toggle {
      display: flex;
      align-items: center;
      justify-content: space-between;
      width: 100%;
      background: transparent;
      border: none;
      cursor: pointer;
      padding: 0;
      text-align: left;
    }
    .journals-toggle h3 {
      margin: 0;
      font-size: 1rem;
    }
    .journals-toggle h3 a {
      color: var(--dark);
      font-weight: 600;
    }
    .journals-toggle h3 a:hover {
      color: var(--secondary);
    }
    .journals-toggle .fold {
      transition: transform 0.3s ease;
      color: var(--dark);
    }
    .journals[data-collapsed="true"] .journals-toggle .fold {
      transform: rotate(-90deg);
    }
    .journals-content {
      transition: max-height 0.3s ease, opacity 0.3s ease;
      overflow: hidden;
    }
    .journals[data-collapsed="true"] .journals-content {
      max-height: 0;
      opacity: 0;
    }
    .journals[data-collapsed="false"] .journals-content {
      max-height: 500px;
      opacity: 1;
    }
    .journal-list {
      list-style: none;
      padding: 0;
      margin: 0.5rem 0;
    }
    .journal-list li {
      margin: 0.25rem 0;
      font-size: 0.9rem;
    }
    .journal-list a {
      color: var(--darkgray);
    }
    .journal-list a:hover {
      color: var(--secondary);
    }
    .journal-empty {
      color: var(--gray);
      font-size: 0.85rem;
    }
    .journal-see-more {
      margin-top: 0.5rem;
      font-size: 0.85rem;
    }
  `

  Journals.afterDOMLoaded = `
    document.querySelectorAll('.journals-toggle').forEach(button => {
      button.addEventListener('click', (e) => {
        if (e.target.closest('a')) return; // Don't toggle if clicking the link
        const journals = button.closest('.journals');
        const isCollapsed = journals.getAttribute('data-collapsed') === 'true';
        journals.setAttribute('data-collapsed', !isCollapsed);
        button.setAttribute('aria-expanded', isCollapsed);
      });
    });
  `

  return Journals
}) satisfies QuartzComponentConstructor
