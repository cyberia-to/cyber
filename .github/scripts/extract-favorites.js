// Extracts favorites from logseq/config.edn and writes to favorites.json
const fs = require('fs')
const path = require('path')

const configPath = path.join(__dirname, '../../logseq/config.edn')
const outputPath = path.join(__dirname, '../../favorites.json')

const config = fs.readFileSync(configPath, 'utf8')

// Extract favorites array using regex (EDN format: :favorites ["item1" "item2" ...])
const match = config.match(/:favorites\s+\[([\s\S]*?)\]/)
if (!match) {
  console.log('No favorites found in config.edn')
  fs.writeFileSync(outputPath, JSON.stringify({ favorites: [] }, null, 2))
  process.exit(0)
}

// Parse the items - they're quoted strings
const itemsStr = match[1]
const items = []
const itemRegex = /"([^"]+)"/g
let itemMatch
while ((itemMatch = itemRegex.exec(itemsStr)) !== null) {
  items.push(itemMatch[1])
}

// Convert to slugs (lowercase, spaces to hyphens)
const slugs = items.map(item =>
  item.toLowerCase()
    .replace(/[🫦]/g, '') // Remove emojis
    .trim()
    .replace(/\s+/g, '-')
)

console.log(`Extracted ${slugs.length} favorites:`, slugs)

fs.writeFileSync(outputPath, JSON.stringify({
  favorites: items,
  slugs: slugs
}, null, 2))

console.log(`Written to ${outputPath}`)
