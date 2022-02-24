{
  description = "A sample Gtk Rust";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: {
      defaultPackage =
        with import nixpkgs { system = "${system}"; };
        stdenv.mkDerivation rec {
          name = "gtkrs-${version}";
          pname = "gtkrs";
          version = "0.2.0-20220215";
          src = self;
          buildInputs = rustc.buildInputs ++ [ cargo rustc libiconv openssl pkgconfig ];
          buildPhase = "cargo build --release";
        }
      ;
    })
  ;
}
