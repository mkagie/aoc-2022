{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        rust-packages = with pkgs; [
          cargo
          rustc
          rustfmt
          pre-commit
          rustPackages.clippy
          rust-analyzer
          cargo-generate
        ];

        python = pkgs.python3.withPackages (p: with p; [
          ipython
          matplotlib
          numpy
          scipy
        ]);
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = rust-packages ++ [ python ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
