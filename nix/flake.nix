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
          sha256 = "sha256-XRdrLdi+/D16RqC14ZI9ds9/t9kqP6n7T/Ag78oMJVM=";
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
                darwin.apple_sdk.frameworks.WebKit
                darwin.apple_sdk.frameworks.CoreFoundation
                darwin.apple_sdk.frameworks.Cocoa
              ];
            }
          );
      });
}
