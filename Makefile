.PHONY: *

check:
	`cargo check`

build:
	`cargo build --target thumbv7em-none-eabihf`

release:
	`cargo build --release --target thumbv7em-none-eabihf`

docs:
	`cargo doc`
