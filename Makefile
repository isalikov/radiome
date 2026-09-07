APP := radiome

.DEFAULT_GOAL := help

.PHONY: help build run test fmt clean

help:
	@printf "%s\n" \
		"radiome targets:" \
		"  make build  - build the app" \
		"  make run    - run the app" \
		"  make test   - run tests" \
		"  make fmt    - format code" \
		"  make clean  - remove build artifacts"

build:
	cargo build

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt

clean:
	cargo clean
