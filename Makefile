.PHONY: preprocess build serve dev clean kill copy-config

# Use node version from .node-version
SHELL := /bin/bash
NODE_VERSION := $(shell cat .node-version)
NVM_USE := source $$HOME/.nvm/nvm.sh && nvm use $(NODE_VERSION)

# Kill any existing Quartz server on port 8080
kill:
	-lsof -ti:8080 | xargs kill -9 2>/dev/null || true

# Copy config files to quartz-build
copy-config:
	cp quartz.config.ts quartz-build/
	cp quartz.layout.ts quartz-build/
	cp custom.scss quartz-build/quartz/styles/
	cp path.ts quartz-build/quartz/util/
	# Copy custom components
	cp Favorites.tsx quartz-build/quartz/components/
	cp Journals.tsx quartz-build/quartz/components/
	cp PageTitle.tsx quartz-build/quartz/components/
	cp ContentMeta.tsx quartz-build/quartz/components/
	cp renderPage.tsx quartz-build/quartz/components/
	cp SidebarToggle.tsx quartz-build/quartz/components/
	cp components-index.ts quartz-build/quartz/components/index.ts
	cp spa.inline.ts quartz-build/quartz/components/scripts/
	# Copy custom component styles and scripts
	cp styles/favorites.scss quartz-build/quartz/components/styles/
	cp styles/journals.scss quartz-build/quartz/components/styles/
	cp scripts/favorites.inline.ts quartz-build/quartz/components/scripts/
	cp scripts/journals.inline.ts quartz-build/quartz/components/scripts/

# Preprocess Logseq content to Quartz format and sync to quartz-build
preprocess:
	node .github/scripts/preprocess-logseq.js
	rm -rf quartz-build/content
	cp -r quartz-content quartz-build/content

# Build the Quartz site
build: copy-config preprocess
	$(NVM_USE) && cd quartz-build && npx quartz build

# Serve locally for development (kills existing server first)
serve: kill copy-config
	$(NVM_USE) && cd quartz-build && npx quartz build --serve

# Full dev workflow: preprocess + serve
dev: preprocess serve

# Clean generated content
clean:
	rm -rf quartz-content
	rm -rf quartz-build/public
