{
  inputs = {
    fenix = {
      url = github:nix-community/fenix;
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = github:numtide/flake-utils;
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };
  outputs = { self, fenix, flake-utils, nixpkgs, flake-compat }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust = fenix.packages.${system}.fromToolchainFile {
          dir = ../.;
          sha256 = "sha256-X3jmirc9bAxFcaqYkSy4qNDJukFWF8zE7idveYvpLFE=";
        };
      in
      {
        devShell =
          (pkgs.mkShell.override
            {
              # Override the native C toolchain provided in the development environment from GCC to
              # Clang. Rust's `lld` linker defaults to a Clang linker flavour. It's easier to switch
              # to the Clang toolchain than it is to switch `lld` to the GCC linker flavour.
              stdenv = pkgs.llvmPackages_latest.stdenv;
            }
            {
              packages = with pkgs; [
                ### Project dependencies ###

                rust

                ### Dependency dependencies ###

                cmake # required by xxhash dep
                openssl
                gnuplot # required by criterion benchmark dep

                ### macOS-specific dependencies ###

                libiconv
              ];

              # Rust dependencies that require a C compiler always use the native compiler, but when
              # compiling to `x86_64-unknown-linux-musl` they must use the cross compiler, otherwise
              # they fail to compile.
              #CC_x86_64_unknown_linux_musl =
              #  with pkgs.pkgsCross.musl64.llvmPackages_latest.stdenv;
              #  "${cc}/bin/${cc.targetPrefix}clang";
            }
          );
      });
}
