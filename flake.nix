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
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = rust-packages;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        # checks = {
        #   format = pkgs.runCommand "check-format"
        #     {
        #       buildInputs = rust-packages ++ [
        #         pkgs.nixpkgs-fmt
        #       ];
        #     }
        #     ''
        #       ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
        #       ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
        #       touch $out
        #     '';
        #   clippy = pkgs.runCommand "check-clippy"
        #     {
        #       buildInputs = rust-packages;
        #     }
        #     ''
        #       ${pkgs.rustPackages.clippy}/bin/cargo-clippy --all-targets --all-features -- -D warnings
        #       touch $out
        #     '';
        # };
      });
}
