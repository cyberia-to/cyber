import { QuartzComponentConstructor, QuartzComponentProps } from "./types"
import readingTime from "reading-time"
import { classNames } from "../util/lang"
import { i18n } from "../i18n"
import { JSX } from "preact"
import style from "./styles/contentMeta.scss"

interface ContentMetaOptions {
  /**
   * Whether to display reading time
   */
  showReadingTime: boolean
  showComma: boolean
}

const defaultOptions: ContentMetaOptions = {
  showReadingTime: true,
  showComma: true,
}

export default ((opts?: Partial<ContentMetaOptions>) => {
  // Merge options with defaults
  const options: ContentMetaOptions = { ...defaultOptions, ...opts }

  function ContentMetadata({ cfg, fileData, displayClass }: QuartzComponentProps) {
    const text = fileData.text

    if (text) {
      const segments: (string | JSX.Element)[] = []

      // Display modified date from frontmatter (git dates added during preprocessing)
      if (fileData.dates?.modified) {
        const dateStr = new Date(fileData.dates.modified).toLocaleDateString(cfg.locale, {
          year: 'numeric',
          month: 'short',
          day: 'numeric'
        })
        segments.push(<span>Updated {dateStr}</span>)
      }

      // Display reading time if enabled
      if (options.showReadingTime) {
        const { minutes, words: _words } = readingTime(text)
        const displayedTime = i18n(cfg.locale).components.contentMeta.readingTime({
          minutes: Math.ceil(minutes),
        })
        segments.push(<span>{displayedTime}</span>)
      }

      // Display aliases if present
      const aliases = fileData.frontmatter?.aliases as string[] | undefined
      if (aliases && aliases.length > 0) {
        segments.push(
          <span class="aliases">
            Also known as: {aliases.map((a, i) => (
              <span key={a}>
                <em>{a}</em>{i < aliases.length - 1 ? ", " : ""}
              </span>
            ))}
          </span>
        )
      }

      return (
        <p show-comma={options.showComma} class={classNames(displayClass, "content-meta")}>
          {segments}
        </p>
      )
    } else {
      return null
    }
  }

  ContentMetadata.css = style

  return ContentMetadata
}) satisfies QuartzComponentConstructor
