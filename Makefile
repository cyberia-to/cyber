.PHONY: build serve check clean

PUBLISHER := src/publish

build:
	cd $(PUBLISHER) && cargo run --release -- build ../..

serve:
	cd $(PUBLISHER) && cargo run -- serve ../..

check:
	cd $(PUBLISHER) && cargo run -- check ../..

test:
	cd $(PUBLISHER) && cargo test

clean:
	rm -rf public
