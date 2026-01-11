.PHONY: preprocess build serve dev clean kill copy-config build-rust test setup

# Use node version from .node-version
SHELL := /bin/bash
NODE_VERSION := $(shell cat .node-version)
NVM_USE := source $$HOME/.nvm/nvm.sh && nvm use $(NODE_VERSION)

# Paths to publish-quartz library (external)
LIBRARY := ../publish-quartz
PREPROCESSOR_DIR := $(LIBRARY)/preprocessor
THEME_DIR := $(LIBRARY)/quartz-theme

# Rust preprocessor binary
RUST_PREPROCESSOR := $(PREPROCESSOR_DIR)/target/release/logseq-to-quartz

# Build Rust preprocessor if not exists
$(RUST_PREPROCESSOR):
	cd $(PREPROCESSOR_DIR) && cargo build --release

build-rust: $(RUST_PREPROCESSOR)

# Kill any existing Quartz server on port 8080
kill:
	-lsof -ti:8080 | xargs kill -9 2>/dev/null || true

# Copy config files to quartz-build
copy-config:
	cp $(THEME_DIR)/quartz.config.ts quartz-build/
	cp $(THEME_DIR)/quartz.layout.ts quartz-build/
	cp $(THEME_DIR)/styles/custom.scss quartz-build/quartz/styles/
	cp $(THEME_DIR)/path.ts quartz-build/quartz/util/
	# Copy custom components
	cp $(THEME_DIR)/components/Favorites.tsx quartz-build/quartz/components/
	cp $(THEME_DIR)/components/Journals.tsx quartz-build/quartz/components/
	cp $(THEME_DIR)/components/PageTitle.tsx quartz-build/quartz/components/
	cp $(THEME_DIR)/components/ContentMeta.tsx quartz-build/quartz/components/
	cp $(THEME_DIR)/components/renderPage.tsx quartz-build/quartz/components/
	cp $(THEME_DIR)/components/SidebarToggle.tsx quartz-build/quartz/components/
	cp $(THEME_DIR)/components/components-index.ts quartz-build/quartz/components/index.ts
	cp $(THEME_DIR)/scripts/spa.inline.ts quartz-build/quartz/components/scripts/
	# Copy custom component styles and scripts
	cp $(THEME_DIR)/styles/favorites.scss quartz-build/quartz/components/styles/
	cp $(THEME_DIR)/styles/journals.scss quartz-build/quartz/components/styles/
	cp $(THEME_DIR)/scripts/favorites.inline.ts quartz-build/quartz/components/scripts/
	cp $(THEME_DIR)/scripts/journals.inline.ts quartz-build/quartz/components/scripts/

# Preprocess Logseq content to Quartz format directly into quartz-build
preprocess: $(RUST_PREPROCESSOR)
	rm -rf quartz-build/content
	$(RUST_PREPROCESSOR) --input . --output quartz-build/content --create-stubs

# Build the Quartz site
build: copy-config preprocess
	$(NVM_USE) && cd quartz-build && npx quartz build

# Serve locally for development (kills existing server first)
serve: kill copy-config
	$(NVM_USE) && cd quartz-build && npx quartz build --serve

# Full dev workflow: preprocess + serve
dev: preprocess serve

# Run preprocessor tests
test:
	cd $(PREPROCESSOR_DIR) && cargo test

# Clone Quartz if not exists and install dependencies
setup:
	@if [ ! -d "quartz-build" ]; then \
		echo "Cloning Quartz..."; \
		git clone https://github.com/jackyzha0/quartz.git quartz-build; \
		cd quartz-build && npm ci; \
	else \
		echo "quartz-build already exists"; \
	fi
	@if [ ! -f "$(RUST_PREPROCESSOR)" ]; then \
		echo "Building preprocessor..."; \
		cd $(PREPROCESSOR_DIR) && cargo build --release; \
	fi

# Clean generated content
clean:
	rm -rf quartz-build/content
	rm -rf quartz-build/public
