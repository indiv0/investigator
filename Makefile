# Installs post-dev-env dependencies & runs the project.
.PHONY: all
all: post-dependencies run

# Installs XCode Command Line Developer Tools.
.PHONY: pre-dependencies
pre-dependencies:
	xcode-select --install

# Activates development environment.
.PHONY: dev-env
dev-env:
	cd nix && \
		nix develop \
			--extra-experimental-features "nix-command flakes" \
			--profile develop

# Updates development environment.
.PHONY: update-dev-env
update-dev-env:
	cd nix && \
		nix flake update \
			--extra-experimental-features "nix-command flakes"

# Installs dependencies needed post-development environment.
.PHONY: post-dependencies
post-dependencies:
	cargo install cargo-watch

# Builds, tests, and runs benchmarks.
.PHONY: run
run:
	cargo run --package utils

# Cleans the Rust project & development environment.
.PHONY: clean
clean:
	cargo clean
	rm -rf \
		nix/develop* \
		investigator/benches/random_data
