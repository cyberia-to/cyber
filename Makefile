.PHONY: build site serve check test release

build:
	cargo build --release --locked

site:
	nu scripts/build.nu

serve:
	nu scripts/serve.nu

check:
	cargo fmt --check
	cargo check --locked

test:
	cargo test --locked

release:
	nu scripts/release.nu
