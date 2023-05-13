# Installs post-dev-env dependencies & runs the project.
.PHONY: all
all: post-dependencies run

# Installs XCode Command Line Developer Tools.
.PHONY: pre-dependencies
pre-dependencies:
	xcode-select --install

# Updates development environment.
.PHONY: update-dev-env
update-dev-env:
	cd nix && \
		nix flake update \
			--extra-experimental-features "nix-command flakes"

# Activates development environment.
.PHONY: dev-env
dev-env:
	cd nix && \
		nix develop \
			--extra-experimental-features "nix-command flakes" \
			--profile develop

# Installs dependencies needed post-development environment.
.PHONY: post-dependencies
post-dependencies:
	cargo install cargo-watch

# Builds, tests, and runs benchmarks.
.PHONY: run
run:
	cargo run --package utils

# Runs the find-files desktop app.
.PHONY: run-find-files
run-find-files:
	(cd packages/find-files-desktop && cargo run --release --package find-files-desktop ~/Desktop/files)

# Continually rebuilds the project.
.PHONY: watch
watch:
	cargo watch -x "run --bin utils" -i dupdir/out

# Cleans the Rust project & development environment.
.PHONY: clean
clean:
	cargo clean
	rm -rf \
		nix/develop* \
		investigator/benches/random_data
