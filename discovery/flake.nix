{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
      in {
        devShell =
          pkgs.mkShell
          {
            nativeBuildInputs = with pkgs; [
              (
                rust-bin.stable.latest.default.override {
                  targets = ["thumbv7em-none-eabihf"];
                  extensions = ["llvm-tools-preview"];
                }
              )
              gdb
              cargo-binutils
              probe-rs
              minicom
            ];
          };
      }
    );
}
