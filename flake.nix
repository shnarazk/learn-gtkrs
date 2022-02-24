{
  description = "An Gtk-rs program";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: {
      defaultPackage =
        with import nixpkgs { system = "${system}"; };
        stdenv.mkDerivation rec {
          name = "gtkrs-example-${version}";
          pname = "gtkrs-example";
          version = "0.2.0-20220224";
          src = self;
          buildInputs = rustc.buildInputs ++ [ cargo rustc pkgconfig gtk4 ];
          buildPhase = "cargo build --release";
           installPhase = ''
              mkdir -p $out/bin;
              cp target/release/gtkrs $out/bin/gtkrs-sample
          '';
       }
      ;
    })
  ;
}
