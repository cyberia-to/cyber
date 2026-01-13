#!/bin/sh

# Use BUILD_DIR env var or default to quartz-build/public
BUILD_DIR="${BUILD_DIR:-$GITHUB_WORKSPACE/quartz-build/public}"

metas='<meta property="og:title" content="cyber docs">

<meta name="description" content="cyber docs">
<meta property="og:description" content="cyber docs">

<meta property="og:image" content="/static/img/logo.png">

<!-- Privacy-friendly analytics by Plausible -->
<script defer data-domain="cyber.page" src="https://plausible.io/js/script.js"></script>'

# Add to all HTML files
find "$BUILD_DIR" -name "*.html" | while read -r file; do
  if [ -f "$file" ]; then
    sed -i "/<\/head>/i\\
$(echo "$metas" | sed 's/$/\\n/' | tr -d '\n')\\
" "$file"
    echo "Added meta tags to $file"
  fi
done
