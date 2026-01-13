#!/bin/sh

# Use BUILD_DIR env var or default to quartz-build/public
BUILD_DIR="${BUILD_DIR:-$GITHUB_WORKSPACE/quartz-build/public}"

metas='<meta property="og:title" content="cyber docs">

<meta name="description" content="cyber docs">
<meta property="og:description" content="cyber docs">

<meta property="og:image" content="/static/img/logo.png">

<!-- Privacy-friendly analytics by Plausible -->
<script async src="https://plausible.io/js/pa-Q95R4OPpKf6e0wpViwLqF.js"></script>
<script>window.plausible=window.plausible||function(){(plausible.q=plausible.q||[]).push(arguments)},plausible.init=plausible.init||function(i){plausible.o=i||{}};plausible.init()</script>'

file="$BUILD_DIR/index.html"

if [ -f "$file" ]; then
  sed -i "/<\/head>/i\\
$(echo "$metas" | sed 's/$/\\n/' | tr -d '\n')\\
" "$file"
  echo "Added meta tags to $file"
else
  echo "Warning: $file not found"
fi
