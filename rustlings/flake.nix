{
  description = "Small exercises to get you used to reading and writing Rust code";

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      cargoBuildInputs = with pkgs;
        lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.CoreServices
        ];

      rustlings = pkgs.rustPlatform.buildRustPackage {
        pname = "rustlings";
        version = "6.0.0";
        doCheck = false;

        src = pkgs.fetchFromGitHub {
          owner = "rust-lang";
          repo = "rustlings";
          rev = "v6.0.0";
          hash = "sha256-KclyTvGyH4EO10rl+kBshPTj+1/8PxDRJ9t100z0nrE=";
        };

        cargoHash = "sha256-p6bSRZopndKYr+E8XQfk8H/AlnzqUEMosR9ml1VRp/k=";
      };
    in {
      devShell = pkgs.mkShell {
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        buildInputs = with pkgs;
          [
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
            rustlings
          ]
          ++ cargoBuildInputs;
      };
      apps = let
        rustlings-app = {
          type = "app";
          program = "${rustlings}/bin/rustlings";
        };
      in {
        default = rustlings-app;
        rustlings = rustlings-app;
      };
      packages = {
        inherit rustlings;
        default = rustlings;
      };
    });
}
