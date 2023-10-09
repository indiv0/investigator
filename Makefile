CARGO := cargo --color=always

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

# Checks, tests, and builds project.
.PHONY: run
run:
	$(CARGO) check
	$(CARGO) test
	$(CARGO) build --release

# Runs benchmarks.
.PHONY: bench
bench:
	cargo bench

# Continually rebuilds the project.
.PHONY: watch
watch:
	cargo watch --shell "make run" -i dupdir/out

# Cleans the Rust project & development environment.
.PHONY: clean
clean:
	cargo clean
	rm -rf \
		nix/develop* \
		dupdir_hash/benches/random_data
