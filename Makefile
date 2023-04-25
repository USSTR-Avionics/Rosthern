.PHONY: *

build:
	`cargo build --target thumbv7em-none-eabihf`

docs:
	`cargo doc`
