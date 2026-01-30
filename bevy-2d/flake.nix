{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/b58df7fc9d5f02c269091f2b0b81a6e06fc859bb";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              # openssl
              alsa-lib
              # Cross Platform 3D Graphics API
              vulkan-loader
              # For debugging around vulkan
              vulkan-tools
              # Other dependencies
              libudev-zero
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              libxkbcommon
              wayland

              # wasm
              wasm-bindgen-cli
              wasm-pack
              binaryen

              pkg-config
              cargo-watch
              rust-analyzer
              (
                rust-bin.stable.latest.default.override {
                  extensions = ["rust-src"];
                  targets = ["wasm32-unknown-unknown"];
                }
              )
            ];
          };
        }
    );
}
