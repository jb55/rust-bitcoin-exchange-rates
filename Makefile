
all: fake
	cargo build

check:
	cargo test -- --nocapture

.PHONY: fake

