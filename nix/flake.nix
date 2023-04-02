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
                ### cloud-v2 dependencies ####

                rust
                cmake # required by xxhash dep
                openssl

                #awscli2
                #google-cloud-sdk
                #terraform

                #awslogs
                #nixpkgs-fmt

                #nodejs

                ## Provides `sops` CLI tool for decrypting and editing `secrets.enc.npekin.yaml` and other files.
                #sops



                #### Enso IDE dependencies ###

                #graalvm11-ce
                #python2



                #### ensogl dependencies ###

                #binaryen
                #pkg-config
                #openssl
                #chromedriver


                #### macOS-specific dependencies ###

                libiconv
                #darwin.apple_sdk.frameworks.Cocoa
                #darwin.apple_sdk.frameworks.Security
                #curl
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
