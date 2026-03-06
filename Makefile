.PHONY: build serve check clean

ROOT    := $(shell pwd)
PUBLISHER := render

build:
	cd $(PUBLISHER) && cargo run --release -- build $(ROOT)

serve:
	cd $(PUBLISHER) && cargo run -- serve $(ROOT)

check:
	cd $(PUBLISHER) && cargo run -- check $(ROOT)

test:
	cd $(PUBLISHER) && cargo test

clean:
	rm -rf build
