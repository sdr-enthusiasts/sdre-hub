# in flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libPath =
          with pkgs;
          lib.makeLibraryPath [
            gtk3
            libsoup_3
            webkitgtk_4_1
          ];

        rustToolchain = (
          pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          }
        );

        # new! 👇
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          cairo
          pango
          atkmm
          gdk-pixbuf
          gtk3
          libsoup_3
          webkitgtk_4_1
          cargo-tauri
          trunk
          dart-sass
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LD_LIBRARY_PATH = libPath;
      in
      with pkgs;
      {
        devShells.default = mkShell {
          # 👇 and now we can just inherit them
          inherit nativeBuildInputs RUST_SRC_PATH LD_LIBRARY_PATH;
        };
      }
    );
}

# https://www.reddit.com/r/rust/comments/mmbfnj/nixifying_a_rust_project/
