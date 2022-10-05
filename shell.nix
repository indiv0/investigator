{}:
# Last successful build of nixpkgs-unstable as of 2022-10-05.
#with import
#  (builtins.fetchTarball {
#    url = "https://github.com/NixOS/nixpkgs/archive/0490b307e5556a8804e710d0c744d29c80fbce48.tar.gz";
#    sha256 = "14d90d8g6kbg5hz7vzwd8v4zh667rk929jdkab3ad065p6s6f5iv";
#  })
#{};
let
  sources = import
    (builtins.fetchTarball {
      url = "https://github.com/NixOS/nixpkgs/archive/0490b307e5556a8804e710d0c744d29c80fbce48.tar.gz";
      sha256 = "14d90d8g6kbg5hz7vzwd8v4zh667rk929jdkab3ad065p6s6f5iv";
    })
  {};
  pkgs = import sources.nixpkgs {};
  #pkgs = import <nixpkgs-unstable> {};
  apple_sdk = pkgs.darwin.apple_sdk;
  frameworks = pkgs.darwin.apple_sdk.frameworks;
  stdenv = pkgs.darwin.apple_sdk.stdenv;
in stdenv.mkDerivation {
  name = "investigator";

  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    frameworks.Security
    frameworks.CoreFoundation
    frameworks.CoreServices
    # Necessary to build `cargo-watch`.
    pkgs.libiconv
  ];

    #export PS1="[$name] \[$txtgrn\]\u@\h\[$txtwht\]:\[$bldpur\]\w \[$txtcyn\]\$git_branch\[$txtred\]\$git_dirty \[$bldylw\]\$aws_env\[$txtrst\]\$ "
  shellHook = ''
    export NIX_LDFLAGS="-F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";
  '';
}
