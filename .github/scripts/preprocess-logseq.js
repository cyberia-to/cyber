#!/usr/bin/env node

/**
 * Preprocessing script to convert Logseq markdown files to Quartz-compatible format.
 *
 * Transformations:
 * - Publish all pages EXCEPT those with private:: true
 * - Convert Logseq properties (key:: value) to YAML frontmatter
 * - Convert {{embed [[page]]}} to ![[page]] (Quartz transclusion)
 * - Execute {{query ...}} and replace with actual page links
 * - Remove inline properties (collapsed::, logseq.order-list-type::, id::)
 * - Copy files from pages/ to content/
 * - Copy assets/ to content/
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Global page index for query execution
let PAGE_INDEX = [];

const SOURCE_DIR = path.join(__dirname, '../../pages');
const JOURNALS_DIR = path.join(__dirname, '../../journals');
const OUTPUT_DIR = path.join(__dirname, '../../quartz-content');
const JOURNALS_OUTPUT = path.join(OUTPUT_DIR, 'journals');
const ASSETS_SOURCE = path.join(__dirname, '../../assets');
const ASSETS_OUTPUT = path.join(OUTPUT_DIR, 'assets');

// Properties that should be converted to frontmatter
const FRONTMATTER_PROPERTIES = ['tags', 'alias', 'title', 'icon', 'public', 'private', 'date', 'description'];

// Properties to remove entirely (inline block properties and query config)
// Match properties at start of line or after list markers (- )
const INLINE_PROPERTIES_REGEX = /^(\s*)(?:-\s*)?(collapsed|logseq\.order-list-type|id|query-table|query-sort-by|query-sort-desc|query-properties):: .+$/gm;

/**
 * Get git dates for a file
 * Returns { modified: 'YYYY-MM-DD', created: 'YYYY-MM-DD' } or null if not tracked
 */
function getGitDates(filepath) {
  try {
    // Get last modified date (most recent commit that touched this file)
    const modified = execSync(
      `git log -1 --format=%aI -- "${filepath}"`,
      { encoding: 'utf-8', cwd: path.dirname(filepath) }
    ).trim();

    // Get created date (first commit that added this file)
    const created = execSync(
      `git log --diff-filter=A --format=%aI -- "${filepath}"`,
      { encoding: 'utf-8', cwd: path.dirname(filepath) }
    ).trim();

    if (!modified) return null;

    // Convert to YYYY-MM-DD format
    return {
      modified: modified.split('T')[0],
      created: created ? created.split('T')[0] : modified.split('T')[0]
    };
  } catch (err) {
    // File not tracked by git or git not available
    return null;
  }
}

/**
 * Parse Logseq properties from the beginning of content
 * Logseq uses "key:: value" format in the first block
 */
function parseLogseqProperties(content) {
  const properties = {};
  const lines = content.split('\n');
  let propertyEndIndex = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    // Logseq properties are at the start, possibly with leading "- " for first block
    const cleanLine = line.replace(/^-\s*/, '').trim();

    // Match property pattern: key:: value
    const match = cleanLine.match(/^([a-zA-Z_-]+)::\s*(.+)$/);

    if (match) {
      const [, key, value] = match;
      properties[key.toLowerCase()] = value.trim();
      propertyEndIndex = i + 1;
    } else if (cleanLine === '' && Object.keys(properties).length > 0) {
      // Empty line after properties, continue checking
      propertyEndIndex = i + 1;
    } else if (Object.keys(properties).length > 0) {
      // Non-property, non-empty line found after properties started
      break;
    } else if (cleanLine !== '' && !cleanLine.startsWith('-')) {
      // Content started without properties
      break;
    }
  }

  // Get remaining content after properties
  const remainingContent = lines.slice(propertyEndIndex).join('\n');

  return { properties, remainingContent };
}

/**
 * Parse aliases, handling wikilinks and comma separation
 * e.g., "[[the new, yet to be born, force]], one simple protocol"
 * becomes ["the new, yet to be born, force", "one simple protocol"]
 */
function parseAliases(aliasString) {
  const aliases = [];
  let current = '';
  let inWikilink = false;

  for (let i = 0; i < aliasString.length; i++) {
    const char = aliasString[i];
    const nextChar = aliasString[i + 1];

    if (char === '[' && nextChar === '[') {
      inWikilink = true;
      i++; // Skip next [
      continue;
    }

    if (char === ']' && nextChar === ']') {
      inWikilink = false;
      i++; // Skip next ]
      continue;
    }

    if (char === ',' && !inWikilink) {
      const trimmed = current.trim();
      if (trimmed) {
        aliases.push(trimmed);
      }
      current = '';
      continue;
    }

    current += char;
  }

  // Add final alias
  const trimmed = current.trim();
  if (trimmed) {
    aliases.push(trimmed);
  }

  return aliases;
}

/**
 * Convert properties object to YAML frontmatter string
 * @param {Object} properties - Logseq properties
 * @param {string} filename - Source filename
 * @param {Object} gitDates - Git dates { modified, created } or null
 */
function toYamlFrontmatter(properties, filename, gitDates = null) {
  // Convert filename to title: extract just the filename part, replace _ with space
  // Filename may contain / for namespaces (e.g., oracle/ask.md)
  const baseName = path.basename(filename, '.md');
  let baseTitle = properties.title || baseName.replace(/_/g, ' ');

  // Prepend icon to title if present
  const icon = properties.icon;
  const title = icon ? `${icon} ${baseTitle}` : baseTitle;

  const frontmatter = {
    title: title,
  };

  // Add tags if present
  if (properties.tags) {
    // Logseq tags can be comma-separated or space-separated
    const tags = properties.tags.split(/[,\s]+/).map(t => t.trim()).filter(Boolean);
    if (tags.length > 0) {
      frontmatter.tags = tags;
    }
  }

  // Add aliases if present
  if (properties.alias) {
    const aliases = parseAliases(properties.alias);
    if (aliases.length > 0) {
      frontmatter.aliases = aliases;
    }
  }

  // Add description if present
  if (properties.description) {
    frontmatter.description = properties.description;
  }

  // Add date if present
  if (properties.date) {
    frontmatter.date = properties.date;
  }

  // Add icon if present (for favorites display)
  if (properties.icon) {
    frontmatter.icon = properties.icon;
  }

  // Build YAML string
  let yaml = '---\n';
  yaml += `title: "${frontmatter.title.replace(/"/g, '\\"')}"\n`;

  // Add icon first (appears after title)
  if (frontmatter.icon) {
    yaml += `icon: "${frontmatter.icon}"\n`;
  }

  if (frontmatter.tags) {
    yaml += `tags:\n${frontmatter.tags.map(t => `  - ${t}`).join('\n')}\n`;
  }

  if (frontmatter.aliases) {
    yaml += `aliases:\n${frontmatter.aliases.map(a => `  - ${a}`).join('\n')}\n`;
  }

  if (frontmatter.description) {
    yaml += `description: "${frontmatter.description.replace(/"/g, '\\"')}"\n`;
  }

  if (frontmatter.date) {
    yaml += `date: ${frontmatter.date}\n`;
  }

  // Add git dates if available
  if (gitDates) {
    yaml += `modified: ${gitDates.modified}\n`;
    yaml += `created: ${gitDates.created}\n`;
  }

  yaml += '---\n\n';

  return yaml;
}

/**
 * Build page index from all source files for query execution
 */
function buildPageIndex(sourceDir) {
  const index = [];
  const files = fs.readdirSync(sourceDir).filter(f => f.endsWith('.md'));

  for (const file of files) {
    const filepath = path.join(sourceDir, file);
    const content = fs.readFileSync(filepath, 'utf-8');
    const { properties } = parseLogseqProperties(content);

    // Skip private pages
    if (properties.private === 'true') continue;

    // Also extract inline/block properties from content (key:: value)
    // This captures properties like "- revenue:: $100" in the body
    const inlineProps = content.match(/^[\s-]*(\w+[-\w]*):: (.+)$/gm);
    if (inlineProps) {
      for (const prop of inlineProps) {
        const match = prop.match(/(\w+[-\w]*):: (.+)/);
        if (match) {
          const key = match[1].toLowerCase();
          const value = match[2].trim();
          // Don't overwrite page-level properties, just add if not exists
          if (!properties[key]) {
            properties[key] = value;
          }
        }
      }
    }

    // Get git dates and add to properties for query tables
    const gitDates = getGitDates(filepath);
    if (gitDates) {
      properties['updated-at'] = gitDates.modified;
      properties['created-at'] = gitDates.created;
    }

    // Parse tags - Logseq tags can be [[tag]] or just tag
    let tags = [];
    if (properties.tags) {
      // Extract tags, handling both [[tag]] and plain tag formats
      const tagStr = properties.tags;
      const tagMatches = tagStr.match(/\[\[([^\]]+)\]\]/g);
      if (tagMatches) {
        tags = tagMatches.map(t => t.replace(/\[\[|\]\]/g, '').toLowerCase());
      } else {
        tags = tagStr.split(/[,\s]+/).map(t => t.trim().toLowerCase()).filter(Boolean);
      }
    }

    // Page name from filename (without extension, with namespace)
    const pageName = file.replace('.md', '').replace(/___/g, '/');
    const pageNameLower = pageName.toLowerCase();

    // Get namespace (folder part before the last /)
    const namespace = pageName.includes('/') ? pageName.split('/').slice(0, -1).join('/') : null;

    index.push({
      name: pageName,
      nameLower: pageNameLower,
      filename: file,
      tags: tags,
      properties: properties,
      namespace: namespace,
      content: content.toLowerCase() // For full-text search
    });
  }

  return index;
}

/**
 * Parse a Logseq query expression and return matching pages
 * Supports: page-tags, property, page-property, and, or, not, namespace
 */
function executeQuery(queryStr) {
  // Extract the query content from {{query ...}}
  const match = queryStr.match(/\{\{query\s+([\s\S]*?)\}\}/i);
  if (!match) return [];

  const query = match[1].trim();

  try {
    return evaluateQuery(query);
  } catch (err) {
    console.error(`Query error: ${err.message} in: ${queryStr.substring(0, 80)}`);
    return [];
  }
}

/**
 * Evaluate a parsed query expression
 */
function evaluateQuery(expr) {
  expr = expr.trim();

  // Extract from {{query ...}} format if wrapped
  const queryMatch = expr.match(/^\{\{query\s+([\s\S]*?)\}\}$/i);
  if (queryMatch) {
    expr = queryMatch[1].trim();
  }

  // Skip empty expressions
  if (!expr || expr === '(and)' || expr === '(or)' || expr === '(not)') {
    return PAGE_INDEX;
  }

  // Handle (and ...)
  if (expr.startsWith('(and ') || expr === '(and)') {
    return evaluateAnd(expr);
  }

  // Handle (or ...)
  if (expr.startsWith('(or ') || expr === '(or)') {
    return evaluateOr(expr);
  }

  // Handle (not ...) - standalone not returns empty (used within and/or)
  if (expr.startsWith('(not ')) {
    return evaluateNot(expr);
  }

  // Handle (task TODO) or (task DONE) etc.
  const taskMatch = expr.match(/^\(task\s+(\w+)\)$/i);
  if (taskMatch) {
    const taskState = taskMatch[1].toUpperCase();
    return PAGE_INDEX.filter(p => p.content.includes(`- ${taskState} `) || p.content.includes(`\n${taskState} `));
  }

  // Handle (page [[name]]) - exact page match
  const pageMatch = expr.match(/^\(page\s+\[\[([^\]]+)\]\]\)$/i);
  if (pageMatch) {
    const pageName = pageMatch[1].toLowerCase();
    return PAGE_INDEX.filter(p => p.nameLower === pageName);
  }

  // Handle (page-tags [[tag]])
  const pageTagsMatch = expr.match(/^\(page-tags\s+\[\[([^\]]+)\]\]\)$/i);
  if (pageTagsMatch) {
    const tag = pageTagsMatch[1].toLowerCase();
    return PAGE_INDEX.filter(p => p.tags.includes(tag));
  }

  // Handle (property :key value) or (property :key "value") or (page-property :key value)
  const propertyMatch = expr.match(/^\((?:page-)?property\s+:?(\w+[-\w]*)(?:\s+(?:"([^"]+)"|(\S+)))?\)$/i);
  if (propertyMatch) {
    const key = propertyMatch[1].toLowerCase().replace(/-/g, '');
    // Strip [[]] from wikilink-formatted values
    let value = (propertyMatch[2] || propertyMatch[3] || '').toLowerCase().replace(/"/g, '');
    value = value.replace(/^\[\[/, '').replace(/\]\]$/, '');
    return PAGE_INDEX.filter(p => {
      // Try both with and without hyphens
      const propVal = (p.properties[key] || p.properties[key.replace(/-/g, '')] || '').toLowerCase();
      if (!value) {
        // Property exists check
        return propVal !== '';
      }
      return propVal === value || propVal.includes(value);
    });
  }

  // Handle [:page-property key value] syntax
  const bracketPropMatch = expr.match(/^\[:?page-property\s+(\w+)\s+(\w+)\]$/i);
  if (bracketPropMatch) {
    const key = bracketPropMatch[1].toLowerCase();
    const value = bracketPropMatch[2].toLowerCase();
    return PAGE_INDEX.filter(p => {
      const propVal = (p.properties[key] || '').toLowerCase();
      return propVal === value || propVal.includes(value);
    });
  }

  // Handle (namespace [[x]])
  const namespaceMatch = expr.match(/^\(namespace\s+\[\[([^\]]+)\]\]\)$/i);
  if (namespaceMatch) {
    const ns = namespaceMatch[1].toLowerCase();
    return PAGE_INDEX.filter(p => p.namespace && p.namespace.toLowerCase() === ns);
  }

  // Handle [[page]] reference (pages that contain this link or have this name)
  const pageRefMatch = expr.match(/^\[\[([^\]]+)\]\]$/);
  if (pageRefMatch) {
    const pageName = pageRefMatch[1].toLowerCase();
    return PAGE_INDEX.filter(p =>
      p.nameLower === pageName ||
      p.content.includes(`[[${pageName}]]`) ||
      p.content.includes(`[[${pageRefMatch[1]}]]`)
    );
  }

  // Handle "text" full-text search
  const textMatch = expr.match(/^"([^"]+)"$/);
  if (textMatch) {
    const searchText = textMatch[1].toLowerCase();
    return PAGE_INDEX.filter(p => p.content.includes(searchText));
  }

  // Handle plain text search (unquoted)
  if (!expr.startsWith('(') && !expr.startsWith('[')) {
    const searchText = expr.toLowerCase().replace(/['"]/g, '');
    if (searchText.length > 2) {
      return PAGE_INDEX.filter(p => p.content.includes(searchText));
    }
  }

  return [];
}

/**
 * Parse and evaluate (and ...) expressions
 */
function evaluateAnd(expr) {
  // Handle empty (and)
  if (expr === '(and)' || expr === '(and )') return PAGE_INDEX;

  // Remove outer (and ) and parse inner expressions
  const inner = expr.slice(5, -1).trim();
  const parts = parseExpressionParts(inner);

  if (parts.length === 0) return PAGE_INDEX;

  let results = PAGE_INDEX;
  for (const part of parts) {
    if (!part.trim()) continue;
    // Skip empty expressions
    if (part === '(and)' || part === '(or)' || part === '(not)') continue;

    const partResults = evaluateQuery(`{{query ${part}}}`);
    const partNames = new Set(partResults.map(p => p.nameLower));
    results = results.filter(p => partNames.has(p.nameLower));
  }

  return results;
}

/**
 * Parse and evaluate (or ...) expressions
 */
function evaluateOr(expr) {
  const inner = expr.slice(4, -1).trim();
  const parts = parseExpressionParts(inner);

  const resultMap = new Map();
  for (const part of parts) {
    if (!part.trim()) continue;
    const partResults = evaluateQuery(`{{query ${part}}}`);
    for (const page of partResults) {
      resultMap.set(page.nameLower, page);
    }
  }

  return Array.from(resultMap.values());
}

/**
 * Parse and evaluate (not ...) expressions
 * Returns pages that DON'T match the inner expression
 */
function evaluateNot(expr) {
  let inner = expr.slice(5, -1).trim();
  if (!inner) return PAGE_INDEX;

  // Handle malformed expressions like "(page [[x]]) (not)" - take only the first valid part
  const parts = parseExpressionParts(inner);
  if (parts.length > 0) {
    // Use only the first valid expression, ignore trailing garbage
    inner = parts[0];
  }

  const excludeResults = evaluateQuery(`{{query ${inner}}}`);
  const excludeNames = new Set(excludeResults.map(p => p.nameLower));
  // Return pages that are NOT in the exclude set
  return PAGE_INDEX.filter(p => !excludeNames.has(p.nameLower));
}

/**
 * Parse expression parts, respecting nested parentheses
 */
function parseExpressionParts(expr) {
  const parts = [];
  let current = '';
  let depth = 0;
  let inBracket = 0;

  for (let i = 0; i < expr.length; i++) {
    const char = expr[i];

    if (char === '(' && expr[i+1] !== ')') {
      depth++;
      current += char;
    } else if (char === ')') {
      depth--;
      current += char;
      if (depth === 0 && current.trim()) {
        parts.push(current.trim());
        current = '';
      }
    } else if (char === '[') {
      inBracket++;
      current += char;
    } else if (char === ']') {
      inBracket--;
      current += char;
    } else if (char === ' ' && depth === 0 && inBracket === 0 && current.trim()) {
      // Space at top level - might be separator
      // But only split if current looks complete
      if (current.match(/^\([^)]+\)$/) || current.match(/^\[\[[^\]]+\]\]$/)) {
        parts.push(current.trim());
        current = '';
      } else {
        current += char;
      }
    } else {
      current += char;
    }
  }

  if (current.trim()) {
    parts.push(current.trim());
  }

  return parts.filter(p => p && p !== '(and)' && p !== '(or)' && p !== '(not)');
}

/**
 * Convert query results to markdown (list or table)
 * @param {Array} results - Matching pages
 * @param {string} queryStr - Original query string
 * @param {Array} columns - Optional column properties for table format
 * @param {string} sortBy - Optional sort column
 * @param {boolean} sortDesc - Sort descending
 */
function queryResultsToMarkdown(results, queryStr, columns = null, sortBy = null, sortDesc = false) {
  if (results.length === 0) {
    return `> [!info] Query Results\n> No pages match this query.\n> \`${queryStr.substring(0, 80)}${queryStr.length > 80 ? '...' : ''}\``;
  }

  // Sort results
  if (sortBy && sortBy !== 'page') {
    results.sort((a, b) => {
      const aVal = (a.properties[sortBy] || '').toString();
      const bVal = (b.properties[sortBy] || '').toString();
      const cmp = aVal.localeCompare(bVal, undefined, { numeric: true });
      return sortDesc ? -cmp : cmp;
    });
  } else {
    // Default: sort by name
    results.sort((a, b) => {
      const cmp = a.name.localeCompare(b.name);
      return sortDesc ? -cmp : cmp;
    });
  }

  // If columns specified, render as table
  if (columns && columns.length > 0) {
    return queryResultsToTable(results, columns);
  }

  // Default: render as list
  const links = results.map(p => {
    const icon = p.properties.icon || '';
    const title = p.properties.title || p.name.replace(/_/g, ' ');
    return `- [[${p.name}|${icon ? icon + ' ' : ''}${title}]]`;
  });

  return links.join('\n');
}

/**
 * Render query results as a markdown table
 */
function queryResultsToTable(results, columns) {
  // Map column names to display headers
  const headerMap = {
    'page': 'Page',
    'block': 'Block',
    'updated-at': 'Updated',
    'created-at': 'Created',
    'tags': 'Tags',
    'alias': 'Alias',
    'size': 'Size',
    'shape': 'Shape',
    'supply': 'Supply',
    'margin': 'Margin',
    'autonomy': 'Autonomy',
    'abundance': 'Abundance',
    'status': 'Status',
    'type': 'Type',
  };

  // Build header row
  const headers = columns.map(col => headerMap[col] || col.charAt(0).toUpperCase() + col.slice(1));
  const headerRow = '| ' + headers.join(' | ') + ' |';
  const separatorRow = '|' + columns.map(() => ' --- ').join('|') + '|';

  // Build data rows
  const dataRows = results.map(p => {
    const cells = columns.map(col => {
      if (col === 'page') {
        const icon = p.properties.icon || '';
        const title = p.properties.title || p.name.replace(/_/g, ' ');
        return `[[${p.name}|${icon ? icon + ' ' : ''}${title}]]`;
      } else if (col === 'block') {
        return ''; // Block content not available in static export
      } else if (col === 'tags') {
        return p.tags.join(', ');
      } else {
        return p.properties[col] || '';
      }
    });
    return '| ' + cells.join(' | ') + ' |';
  });

  return [headerRow, separatorRow, ...dataRows].join('\n');
}

/**
 * Parse query-properties from content following a query
 * Returns { columns: [...], sortBy: string, sortDesc: boolean }
 */
function parseQueryOptions(content, startIndex) {
  const options = { columns: null, sortBy: null, sortDesc: false };

  // Look at next few lines for query options
  const lines = content.substring(startIndex).split('\n').slice(0, 5);

  for (const line of lines) {
    // Match query-properties:: [:page :column1 :column2]
    const propsMatch = line.match(/query-properties::\s*\[([^\]]+)\]/i);
    if (propsMatch) {
      options.columns = propsMatch[1]
        .split(/\s+/)
        .map(c => c.replace(/^:/, '').trim())
        .filter(Boolean);
    }

    // Match query-sort-by:: column
    const sortMatch = line.match(/query-sort-by::\s*(\S+)/i);
    if (sortMatch) {
      options.sortBy = sortMatch[1].replace(/^:/, '');
    }

    // Match query-sort-desc:: true/false
    const descMatch = line.match(/query-sort-desc::\s*(true|false)/i);
    if (descMatch) {
      options.sortDesc = descMatch[1].toLowerCase() === 'true';
    }
  }

  return options;
}

/**
 * Convert Logseq-style tables to proper GFM tables
 * Logseq tables: lines starting with | after list markers, without separator row
 * GFM tables: need |---|---| separator after header row
 */
function convertLogseqTables(content) {
  const lines = content.split('\n');
  const result = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];
    // Check if this line starts a table (list item with pipe or just pipe)
    const tableMatch = line.match(/^(\s*)(?:-\s*)?\|(.+)\|(\s*)$/);

    if (tableMatch) {
      // Found start of a table, collect all table rows
      const indent = tableMatch[1] || '';
      const tableRows = [];

      // Collect table rows, skipping blank lines between them
      while (i < lines.length) {
        const currentLine = lines[i];

        // Skip blank lines (they might separate table rows in Logseq)
        if (currentLine.trim() === '') {
          // Look ahead to see if there's another table row
          let nextIdx = i + 1;
          while (nextIdx < lines.length && lines[nextIdx].trim() === '') {
            nextIdx++;
          }
          if (nextIdx < lines.length && lines[nextIdx].match(/^(\s*)(?:-\s*)?\|(.+)\|(\s*)$/)) {
            i++;
            continue;
          } else {
            break;
          }
        }

        // Match table row: optional indent, optional list marker, pipe content
        const rowMatch = currentLine.match(/^(\s*)(?:-\s*)?\|(.+)\|(\s*)$/);

        if (rowMatch) {
          // Extract just the table content (between pipes)
          const cells = rowMatch[2];
          tableRows.push(`|${cells}|`);
          i++;
        } else {
          break;
        }
      }

      if (tableRows.length >= 2) {
        // We have a proper table (header + at least one row)
        // Add blank line before table if previous line isn't blank
        if (result.length > 0 && result[result.length - 1].trim() !== '') {
          result.push('');
        }

        // Add the header row
        result.push(tableRows[0]);

        // Count columns and create separator
        const columnCount = (tableRows[0].match(/\|/g) || []).length - 1;
        const separator = '|' + ' --- |'.repeat(columnCount);
        result.push(separator);

        // Add remaining rows
        for (let j = 1; j < tableRows.length; j++) {
          result.push(tableRows[j]);
        }
        result.push(''); // Add empty line after table
      } else if (tableRows.length === 1) {
        // Single row - not a valid table, keep as is
        result.push(tableRows[0]);
      }
    } else {
      result.push(line);
      i++;
    }
  }

  return result.join('\n');
}

/**
 * Convert Logseq-specific syntax to Quartz-compatible format
 */
function convertLogseqSyntax(content) {
  let result = content;

  // Convert Logseq tables to GFM tables first
  result = convertLogseqTables(result);

  // Convert headings in list items to proper markdown headings
  // - ## heading -> ## heading (so TOC can pick them up)
  // Also fix indentation of items following the heading
  const lines = result.split('\n');
  const processedLines = [];
  let indentToRemove = null;

  for (let i = 0; i < lines.length; i++) {
    let line = lines[i];

    // Check for heading in list item: "  - ## heading" or "\t- ## heading"
    const headingMatch = line.match(/^(\s*)-\s+(#{1,6}\s+.+)$/);
    if (headingMatch) {
      const indent = headingMatch[1];
      // Convert to just the heading (remove list prefix)
      line = headingMatch[2];
      // Set indent to remove from following lines (one level deeper than heading was)
      indentToRemove = indent + '\t';
      processedLines.push(line);
      continue;
    }

    // If we have an indent to remove and line starts with it, remove it
    if (indentToRemove && line.startsWith(indentToRemove)) {
      line = line.slice(indentToRemove.length);
    } else if (indentToRemove && !line.startsWith(indentToRemove) && line.trim() !== '') {
      // Line doesn't have expected indent, stop removing
      indentToRemove = null;
    }

    processedLines.push(line);
  }
  result = processedLines.join('\n');

  // Wikilinks with namespaces like [[oracle/ask]] are kept as-is
  // Files are now organized in folders (oracle/ask.md) so links work directly

  // Escape $ within wikilinks to prevent LaTeX interpretation
  // [[$ PUSSY]] -> [[\$PUSSY]], but preserve wikilink structure
  result = result.replace(/\[\[([^\]]*\$[^\]]*)\]\]/g, (match, inner) => {
    // Escape $ signs within the wikilink content
    return '[[' + inner.replace(/\$/g, '\\$') + ']]';
  });

  // Escape standalone $ followed by uppercase letters (likely token names like $PUSSY, $ETH)
  // But not math expressions like $x^2$ or already escaped \$
  result = result.replace(/(?<!\\)\$([A-Z][A-Z0-9]*)/g, (match, name) => '\\$' + name);

  // Convert {{embed [[page]]}} to ![[page]] (Quartz transclusion)
  result = result.replace(/\{\{embed\s+\[\[([^\]]+)\]\]\s*\}\}/gi, '![[$1]]');

  // Convert {{embed ((block-id))}} to placeholder (block UUIDs can't be resolved without Logseq)
  // Use single-line callout to avoid indentation issues when inside list items
  result = result.replace(/\{\{embed\s+\(\(([^)]+)\)\)\s*\}\}/gi, '*Block embed - view in Logseq*');

  // Convert block references ((block-id)) to links
  // These reference specific blocks by their UUID
  result = result.replace(/\(\(([a-f0-9-]{36})\)\)/gi, '[→ block](#^$1)');

  // Execute {{query ...}} and replace with actual results
  // Match multiline queries that can span multiple lines
  // Also capture any leading list marker (- ) to handle queries in lists
  result = result.replace(/^(\s*-\s*)?\{\{query[\s\S]*?\}\}/gim, (match, listMarker, offset) => {
    const queryMatch = match.replace(/^\s*-\s*/, ''); // Remove list marker if present
    const results = executeQuery(queryMatch);
    // Look for query options (query-properties, query-sort-by, etc.) after this query
    const options = parseQueryOptions(result, offset + match.length);
    const output = queryResultsToMarkdown(results, queryMatch, options.columns, options.sortBy, options.sortDesc);

    // If it's a table (has columns) and was in a list, add blank line before to break out of list
    if (options.columns && options.columns.length > 0) {
      return '\n\n' + output;
    }
    // For lists, keep the list marker
    return listMarker ? listMarker + output.split('\n').join('\n' + listMarker) : output;
  });

  // Convert {{youtube URL}} to YouTube embed
  result = result.replace(/\{\{youtube\s+([^\}]+)\}\}/gi, '![$1]($1)');

  // Convert {{video URL}} to video embed
  result = result.replace(/\{\{video\s+([^\}]+)\}\}/gi, '![$1]($1)');

  // Convert {{pdf URL}} to PDF embed
  result = result.replace(/\{\{pdf\s+([^\}]+)\}\}/gi, '![$1]($1)');

  // Convert {{renderer ...}} to placeholder
  result = result.replace(/\{\{renderer\s+[^\}]+\}\}/gi, '`[renderer]`');

  // Convert {{cloze text}} to highlighted text
  result = result.replace(/\{\{cloze\s+([^\}]+)\}\}/gi, '==$1==');

  // Convert Logseq task markers to standard markdown checkboxes
  // DONE -> - [x], TODO -> - [ ], NOW/DOING -> - [ ] 🔄, LATER -> - [ ] 📅
  result = result.replace(/^(\s*)-\s+DONE\s+/gm, '$1- [x] ');
  result = result.replace(/^(\s*)-\s+TODO\s+/gm, '$1- [ ] ');
  result = result.replace(/^(\s*)-\s+NOW\s+/gm, '$1- [ ] 🔄 ');
  result = result.replace(/^(\s*)-\s+DOING\s+/gm, '$1- [ ] 🔄 ');
  result = result.replace(/^(\s*)-\s+LATER\s+/gm, '$1- [ ] 📅 ');
  result = result.replace(/^(\s*)-\s+WAITING\s+/gm, '$1- [ ] ⏳ ');
  result = result.replace(/^(\s*)-\s+CANCELLED\s+/gm, '$1- [x] ❌ ');

  // Convert priority markers [#A], [#B], [#C]
  result = result.replace(/\[#A\]/g, '🔴');
  result = result.replace(/\[#B\]/g, '🟡');
  result = result.replace(/\[#C\]/g, '🟢');

  // Convert SCHEDULED: and DEADLINE: to visible text
  result = result.replace(/SCHEDULED:\s*<([^>]+)>/g, '📅 Scheduled: $1');
  result = result.replace(/DEADLINE:\s*<([^>]+)>/g, '⏰ Deadline: $1');

  // Remove inline block properties
  result = result.replace(INLINE_PROPERTIES_REGEX, '$1');

  // Remove #+BEGIN_... #+END_... blocks (org-mode syntax sometimes in Logseq)
  result = result.replace(/#\+BEGIN_\w+[\s\S]*?#\+END_\w+/gi, '');

  // Remove Logseq image resizing syntax {:height X, :width Y}
  // Handle numeric values and 'auto'
  result = result.replace(/\{:height\s+[\w\d]+,?\s*:?width?\s*[\w\d]*\}/gi, '');
  result = result.replace(/\{:width\s+[\w\d]+,?\s*:?height?\s*[\w\d]*\}/gi, '');

  // Remove :LOGBOOK: blocks (time tracking)
  result = result.replace(/^\s*:LOGBOOK:[\s\S]*?:END:\s*$/gm, '');

  // Remove standalone CLOCK entries
  result = result.replace(/^\s*CLOCK:.*$/gm, '');

  // Clean up multiple empty lines
  result = result.replace(/\n{3,}/g, '\n\n');

  return result;
}

/**
 * Process a single markdown file
 * Returns { published: true/false, properties: count }
 */
function processFile(sourcePath, outputPath) {
  const content = fs.readFileSync(sourcePath, 'utf-8');
  const filename = path.basename(sourcePath);

  // Parse Logseq properties
  const { properties, remainingContent } = parseLogseqProperties(content);

  // Skip pages with private:: true
  if (properties.private === 'true') {
    return { published: false, properties: Object.keys(properties).length };
  }

  // Get git dates for the source file
  const gitDates = getGitDates(sourcePath);

  // Generate YAML frontmatter with git dates
  const frontmatter = toYamlFrontmatter(properties, filename, gitDates);

  // Convert Logseq-specific syntax
  let convertedContent = convertLogseqSyntax(remainingContent);

  // Convert tabs to spaces to prevent markdown from treating indented content as code blocks
  // Logseq uses tabs for indentation, but markdown interprets tabs as code blocks
  convertedContent = convertedContent.replace(/\t/g, '  ');

  // Fix list items that have leading spaces after headings
  // In Logseq, items under headings are indented by one tab (now 2 spaces), but in markdown they shouldn't be
  // We need to "dedent" all content under headings by one level (2 spaces)
  const lines = convertedContent.split('\n');
  const fixedLines = [];
  let inListSection = false;
  let inTable = false;

  for (let i = 0; i < lines.length; i++) {
    let line = lines[i];

    // Headings mark the start of a new section - content after should be dedented
    if (line.match(/^#{1,6}\s/)) {
      inListSection = true;
      inTable = false;
      fixedLines.push(line);
      continue;
    }

    // Check if line is blank (only whitespace)
    if (line.trim() === '') {
      // If we're in a table context, skip blank lines between table rows
      if (inTable) {
        // Check if next non-blank line is a table row
        let nextLineIdx = i + 1;
        while (nextLineIdx < lines.length && lines[nextLineIdx].trim() === '') {
          nextLineIdx++;
        }
        if (nextLineIdx < lines.length && lines[nextLineIdx].trim().startsWith('|')) {
          // Skip this blank line - don't add it
          continue;
        }
      }
      fixedLines.push(line);
      continue;
    }

    // Track table context
    if (line.trim().startsWith('|')) {
      inTable = true;
    } else {
      inTable = false;
    }

    // In a list section, remove one level of indentation (2 spaces) from lines that have it
    if (inListSection && line.startsWith('  ')) {
      line = line.slice(2);
    }

    fixedLines.push(line);
  }

  convertedContent = fixedLines.join('\n');

  // Fix asset paths based on file depth
  // Quartz creates folder/index.html for each .md file, so we need to go up one more level
  // Root file: content/page.md → public/page/index.html → ../assets/ (up from page/, into assets/)
  // Nested file: content/a/b.md → public/a/b/index.html → ../../assets/ (up from b/, up from a/, into assets/)
  const relativePath = path.relative(OUTPUT_DIR, outputPath);
  const depth = relativePath.split(path.sep).length; // 1 for root files, 2 for one level deep, etc.
  const assetPrefix = '../'.repeat(depth) + 'assets/';
  convertedContent = convertedContent.replace(/\]?\(\.\.\/assets\//g, (match) => {
    // Preserve the ]( prefix if present
    const prefix = match.startsWith('](') ? '](' : '(';
    return prefix + assetPrefix;
  });

  // Combine frontmatter and content
  const finalContent = frontmatter + convertedContent;

  // Ensure output directory exists
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  // Write output file
  fs.writeFileSync(outputPath, finalContent);

  return { published: true, properties: Object.keys(properties).length };
}

/**
 * Copy directory recursively
 */
function copyDirectory(source, destination) {
  if (!fs.existsSync(source)) {
    console.log(`Source directory does not exist: ${source}`);
    return 0;
  }

  if (!fs.existsSync(destination)) {
    fs.mkdirSync(destination, { recursive: true });
  }

  let count = 0;
  const entries = fs.readdirSync(source, { withFileTypes: true });

  for (const entry of entries) {
    const sourcePath = path.join(source, entry.name);
    const destPath = path.join(destination, entry.name);

    if (entry.isDirectory()) {
      count += copyDirectory(sourcePath, destPath);
    } else {
      fs.copyFileSync(sourcePath, destPath);
      count++;
    }
  }

  return count;
}

/**
 * Extract all wikilinks from content
 */
function extractWikilinks(content) {
  const links = new Set();
  // Match [[link]] and [[link|alias]] patterns
  const regex = /\[\[([^\]|]+)(?:\|[^\]]+)?\]\]/g;
  let match;
  while ((match = regex.exec(content)) !== null) {
    let link = match[1].trim();
    // Skip external links, anchors, and embeds
    if (link.startsWith('http') || link.startsWith('#') || link.startsWith('!')) continue;
    // Clean up the link
    link = link.replace(/^\.\//, ''); // Remove leading ./
    links.add(link);
  }
  return links;
}

/**
 * Convert a page name to its expected filename
 */
function pageNameToFilename(pageName) {
  // Logseq uses the page name directly as filename (with .md)
  // Special characters are preserved in filenames
  return pageName + '.md';
}

/**
 * Create stub pages for all referenced but missing pages
 */
function createStubPages(outputDir) {
  console.log('\nScanning for missing linked pages...');

  // Recursively get all existing .md files (with relative paths, lowercase for comparison)
  function getAllMdFiles(dir, baseDir = dir) {
    const results = new Set();
    const items = fs.readdirSync(dir, { withFileTypes: true });
    for (const item of items) {
      const fullPath = path.join(dir, item.name);
      if (item.isDirectory()) {
        getAllMdFiles(fullPath, baseDir).forEach(f => results.add(f));
      } else if (item.name.endsWith('.md')) {
        // Store relative path from baseDir
        const relativePath = path.relative(baseDir, fullPath).toLowerCase();
        results.add(relativePath);
      }
    }
    return results;
  }

  const existingFiles = getAllMdFiles(outputDir);

  // Collect all wikilinks from all files (recursively)
  const allLinks = new Set();
  function scanForLinks(dir) {
    const items = fs.readdirSync(dir, { withFileTypes: true });
    for (const item of items) {
      const fullPath = path.join(dir, item.name);
      if (item.isDirectory()) {
        scanForLinks(fullPath);
      } else if (item.name.endsWith('.md')) {
        const content = fs.readFileSync(fullPath, 'utf-8');
        const links = extractWikilinks(content);
        links.forEach(link => allLinks.add(link));
      }
    }
  }
  scanForLinks(outputDir);

  // Find missing pages
  const missingPages = [];
  for (const link of allLinks) {
    // Convert link to file path (namespace / becomes folder separator)
    const filePath = (link + '.md').toLowerCase();

    if (!existingFiles.has(filePath)) {
      // Filter out obvious non-pages
      if (link.match(/^[a-zA-Z]/) && // Starts with letter
          !link.includes('://') && // Not a URL
          !link.startsWith('/') && // Not an API path
          !link.startsWith('\\$') && // Not escaped dollar (token)
          link.length > 1 && // Not single char
          link.length < 200) { // Not too long
        missingPages.push(link);
      }
    }
  }

  console.log(`Found ${missingPages.length} missing linked pages`);

  // Create stub pages
  let created = 0;
  for (const pageName of missingPages) {
    // Clean the page name - remove escape characters that were added during preprocessing
    const cleanPageName = pageName.replace(/\\\$/g, '$').replace(/\\/g, '');

    // Keep namespace as folder structure
    const filename = cleanPageName + '.md';
    const filepath = path.join(outputDir, filename);

    // Skip if file exists
    if (existingFiles.has(filename.toLowerCase())) continue;
    if (fs.existsSync(filepath)) continue;

    // Create stub content with frontmatter
    // Use clean page name for title, preserve namespace separators as /
    let title = cleanPageName.replace(/_/g, ' ');
    // Escape quotes for YAML
    title = title.replace(/"/g, '\\"');
    const stubContent = `---
title: "${title}"
stub: true
---

> [!note] Stub Page
> This page was auto-generated because it was linked from other pages but doesn't have its own content yet.
>
> Check the **Backlinks** section below to see where this page is referenced.
`;

    try {
      // Create directory if needed
      const dir = path.dirname(filepath);
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
      }
      fs.writeFileSync(filepath, stubContent);
      created++;
    } catch (err) {
      // Skip files that can't be created (invalid filenames)
      console.error(`Could not create stub for "${pageName}": ${err.message}`);
    }
  }

  console.log(`Created ${created} stub pages`);
  return created;
}

/**
 * Parse date from journal filename
 * Logseq uses formats like: 2024_08_16.md or 2024-08-16.md
 * Returns { date: 'YYYY-MM-DD', title: 'Month Day, Year' } or null
 */
function parseJournalDate(filename) {
  // Remove .md extension
  const basename = filename.replace('.md', '');

  // Try underscore format: 2024_08_16
  let match = basename.match(/^(\d{4})_(\d{2})_(\d{2})$/);
  if (!match) {
    // Try dash format: 2024-08-16
    match = basename.match(/^(\d{4})-(\d{2})-(\d{2})$/);
  }

  if (!match) return null;

  const [, year, month, day] = match;
  const date = `${year}-${month}-${day}`;

  // Create human-readable title
  const dateObj = new Date(parseInt(year), parseInt(month) - 1, parseInt(day));
  const options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };
  const title = dateObj.toLocaleDateString('en-US', options);

  return { date, title, year, month, day };
}

/**
 * Process a single journal file
 * Returns { published: true/false, date: string }
 */
function processJournalFile(sourcePath, outputPath) {
  const content = fs.readFileSync(sourcePath, 'utf-8');
  const filename = path.basename(sourcePath);

  // Parse date from filename
  const dateInfo = parseJournalDate(filename);
  if (!dateInfo) {
    console.warn(`Skipping journal file with invalid date format: ${filename}`);
    return { published: false, date: null };
  }

  // Parse Logseq properties
  const { properties, remainingContent } = parseLogseqProperties(content);

  // Skip pages with private:: true
  if (properties.private === 'true') {
    return { published: false, date: dateInfo.date };
  }

  // Build frontmatter for journal entry
  let yaml = '---\n';
  yaml += `title: "${dateInfo.title}"\n`;
  yaml += `date: ${dateInfo.date}\n`;
  yaml += `tags:\n  - journal\n`;

  // Add any additional tags from properties
  if (properties.tags) {
    const tags = properties.tags.split(/[,\s]+/).map(t => t.trim()).filter(Boolean);
    for (const tag of tags) {
      if (tag !== 'journal') {
        yaml += `  - ${tag}\n`;
      }
    }
  }

  yaml += '---\n\n';

  // Convert Logseq-specific syntax
  let convertedContent = convertLogseqSyntax(remainingContent);

  // Convert tabs to spaces to prevent markdown from treating indented content as code blocks
  convertedContent = convertedContent.replace(/\t/g, '  ');

  // Fix list items that have leading spaces (same as in processFile)
  // Journal content is at top level, so we start in "dedent mode"
  const lines = convertedContent.split('\n');
  const fixedLines = [];
  let inListSection = true; // Start as true since journal content is at top level

  for (let i = 0; i < lines.length; i++) {
    let line = lines[i];

    if (line.match(/^#{1,6}\s/)) {
      inListSection = true;
      fixedLines.push(line);
      continue;
    }

    if (line.trim() === '') {
      fixedLines.push(line);
      continue;
    }

    // Remove one level of indentation (2 spaces) from lines that have it
    if (inListSection && line.startsWith('  ')) {
      line = line.slice(2);
    }

    fixedLines.push(line);
  }

  convertedContent = fixedLines.join('\n');

  // Fix asset paths (journals are one level deep: journals/2024-08-16.md)
  convertedContent = convertedContent.replace(/\]?\(\.\.\/assets\//g, (match) => {
    const prefix = match.startsWith('](') ? '](' : '(';
    return prefix + '../../assets/';
  });

  // Combine frontmatter and content
  const finalContent = yaml + convertedContent;

  // Ensure output directory exists
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  // Write output file
  fs.writeFileSync(outputPath, finalContent);

  return { published: true, date: dateInfo.date, title: dateInfo.title, content: convertedContent };
}

/**
 * Process all journal files and create journal index
 */
function processJournals() {
  if (!fs.existsSync(JOURNALS_DIR)) {
    console.log('No journals directory found, skipping...');
    return { processed: 0, entries: [] };
  }

  const files = fs.readdirSync(JOURNALS_DIR).filter(f => f.endsWith('.md'));
  console.log(`\nProcessing ${files.length} journal files...`);

  // Ensure journals output directory exists
  if (!fs.existsSync(JOURNALS_OUTPUT)) {
    fs.mkdirSync(JOURNALS_OUTPUT, { recursive: true });
  }

  const entries = [];
  let processed = 0;
  let skipped = 0;

  for (const file of files) {
    const sourcePath = path.join(JOURNALS_DIR, file);
    // Convert underscore to dash for cleaner URLs: 2024_08_16.md -> 2024-08-16.md
    const outputFile = file.replace(/_/g, '-');
    const outputPath = path.join(JOURNALS_OUTPUT, outputFile);

    try {
      const result = processJournalFile(sourcePath, outputPath);
      if (result.published) {
        processed++;
        entries.push({
          date: result.date,
          title: result.title,
          filename: outputFile.replace('.md', ''),
          content: result.content,
        });
      } else {
        skipped++;
      }
    } catch (err) {
      console.error(`Error processing journal ${file}: ${err.message}`);
    }
  }

  console.log(`Published: ${processed} journal entries`);
  console.log(`Skipped: ${skipped} journal entries (private or invalid)`);

  return { processed, entries };
}

/**
 * Create journal index page listing all entries by date
 */
function createJournalIndex(entries) {
  if (entries.length === 0) {
    console.log('No journal entries to index');
    return;
  }

  // Sort entries by date (newest first)
  entries.sort((a, b) => b.date.localeCompare(a.date));

  // Build index content - Logseq style with full content under each date
  let content = `---
title: "Journal"
---

`;

  // Show each entry with date header and full content (like Logseq)
  for (const entry of entries) {
    // Date header as a link to the individual journal page
    content += `## [[journals/${entry.filename}|${entry.title}]]\n\n`;

    // Add the journal content directly
    if (entry.content && entry.content.trim()) {
      content += entry.content.trim() + '\n\n';
    }

    content += '---\n\n';
  }

  // Write index file
  const indexPath = path.join(JOURNALS_OUTPUT, 'index.md');
  fs.writeFileSync(indexPath, content);
  console.log(`Created journal index with ${entries.length} entries`);
}

const FAVORITES_OUTPUT = path.join(OUTPUT_DIR, 'favorites');
const LOGSEQ_CONFIG = path.join(__dirname, '../../logseq/config.edn');

/**
 * Extract favorites from logseq/config.edn
 */
function getFavorites() {
  if (!fs.existsSync(LOGSEQ_CONFIG)) {
    console.log('No logseq/config.edn found, skipping favorites');
    return [];
  }

  const config = fs.readFileSync(LOGSEQ_CONFIG, 'utf8');
  const match = config.match(/:favorites\s+\[([\s\S]*?)\]/);
  if (!match) {
    console.log('No favorites found in config.edn');
    return [];
  }

  const items = [];
  const itemRegex = /"([^"]+)"/g;
  let itemMatch;
  while ((itemMatch = itemRegex.exec(match[1])) !== null) {
    items.push(itemMatch[1]);
  }

  return items;
}

/**
 * Process favorites - create favorites folder with embed files
 */
function processFavorites() {
  const favorites = getFavorites();
  if (favorites.length === 0) return { count: 0 };

  // Create favorites directory
  fs.mkdirSync(FAVORITES_OUTPUT, { recursive: true });

  console.log(`\nProcessing ${favorites.length} favorites...`);

  let created = 0;
  const validFavorites = [];

  for (const fav of favorites) {
    // Convert to slug (lowercase, spaces to hyphens)
    const slug = fav.toLowerCase()
      .replace(/[🫦]/g, '') // Remove emojis
      .trim()
      .replace(/\s+/g, '-');

    // Check if the source page exists (in pages/ folder)
    const sourcePath = path.join(OUTPUT_DIR, 'pages', `${slug}.md`);
    if (!fs.existsSync(sourcePath)) {
      // Try with underscores (old Logseq format)
      const altSlug = slug.replace(/-/g, '_');
      const altPath = path.join(OUTPUT_DIR, 'pages', `${altSlug}.md`);
      if (!fs.existsSync(altPath)) {
        console.log(`  Favorite "${fav}" (${slug}) not found, skipping`);
        continue;
      }
    }

    validFavorites.push({ name: fav, slug });

    // Create embed file for this favorite
    const favPath = path.join(FAVORITES_OUTPUT, `${slug}.md`);
    const icon = PAGE_INDEX.find(p => p.nameLower === slug)?.properties?.icon || '';

    fs.writeFileSync(favPath, `---
title: "${icon ? icon + ' ' : ''}${fav}"
---

![[pages/${slug}]]
`);
    created++;
  }

  // Create favorites index
  if (validFavorites.length > 0) {
    let indexContent = `---
title: "⭐ Favorites"
---

`;
    for (const { name, slug } of validFavorites) {
      const icon = PAGE_INDEX.find(p => p.nameLower === slug)?.properties?.icon || '';
      // Link directly to the page in pages/ folder
      indexContent += `- [[pages/${slug}|${icon ? icon + ' ' : ''}${name}]]\n`;
    }

    fs.writeFileSync(path.join(FAVORITES_OUTPUT, 'index.md'), indexContent);
  }

  console.log(`Created ${created} favorite pages`);
  return { count: created };
}

/**
 * Main processing function
 */
function main() {
  console.log('Preprocessing Logseq content for Quartz...\n');

  // Build page index for query execution
  console.log('Building page index for query execution...');
  PAGE_INDEX = buildPageIndex(SOURCE_DIR);
  console.log(`Indexed ${PAGE_INDEX.length} pages\n`);

  // Clean output directory
  if (fs.existsSync(OUTPUT_DIR)) {
    fs.rmSync(OUTPUT_DIR, { recursive: true });
  }
  fs.mkdirSync(OUTPUT_DIR, { recursive: true });

  // Process all markdown files in pages/
  const files = fs.readdirSync(SOURCE_DIR).filter(f => f.endsWith('.md'));
  let processed = 0;
  let errors = 0;

  console.log(`Processing ${files.length} markdown files from pages/...`);

  // Create "pages" subfolder for all regular pages
  const PAGES_OUTPUT = path.join(OUTPUT_DIR, 'pages');
  fs.mkdirSync(PAGES_OUTPUT, { recursive: true });

  let skipped = 0;
  for (const file of files) {
    const sourcePath = path.join(SOURCE_DIR, file);
    // Convert namespace files (oracle___ask.md) to folder structure (oracle/ask.md)
    const outputFile = file.replace(/___/g, '/');
    // Put all pages in the "pages" subfolder
    const outputPath = path.join(PAGES_OUTPUT, outputFile);

    try {
      const result = processFile(sourcePath, outputPath);
      if (result.published) {
        processed++;
      } else {
        skipped++;
      }
    } catch (err) {
      console.error(`Error processing ${file}: ${err.message}`);
      errors++;
    }
  }

  console.log(`Published: ${processed} files`);
  console.log(`Skipped: ${skipped} files (private:: true)`);
  if (errors > 0) {
    console.log(`Errors: ${errors} files`);
  }

  // Copy assets
  console.log('\nCopying assets...');
  const assetCount = copyDirectory(ASSETS_SOURCE, ASSETS_OUTPUT);
  console.log(`Copied: ${assetCount} asset files`);

  // Process journal files
  const journalResult = processJournals();
  if (journalResult.entries.length > 0) {
    createJournalIndex(journalResult.entries);
  }

  // Process favorites from logseq/config.edn
  processFavorites();

  // Create index.md pointing to pages/cyber.md if it doesn't exist
  const indexPath = path.join(OUTPUT_DIR, 'index.md');
  if (!fs.existsSync(indexPath)) {
    const cyberPath = path.join(OUTPUT_DIR, 'pages/cyber.md');
    if (fs.existsSync(cyberPath)) {
      // Create index that redirects to cyber
      fs.writeFileSync(indexPath, `---
title: "Cyber"
---

![[pages/cyber]]
`);
      console.log('\nCreated index.md pointing to pages/cyber.md');
    }
  }

  // Create pages/index.md listing all pages
  const pagesIndexPath = path.join(OUTPUT_DIR, 'pages/index.md');
  fs.writeFileSync(pagesIndexPath, `---
title: "📚 All Pages"
---

All pages in the knowledge base.
`);

  // Create stub pages for missing linked pages (in the pages folder)
  createStubPages(PAGES_OUTPUT);

  console.log('\nPreprocessing complete!');
  console.log(`Output directory: ${OUTPUT_DIR}`);
}

main();
