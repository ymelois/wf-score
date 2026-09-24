{
  description = "wf-score development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:
    let
      forAllSystems = f: builtins.mapAttrs f nixpkgs.legacyPackages;
    in
    {
      devShells = forAllSystems (
        system: pkgs:
        let
          toolchainFile = fenix.packages."${system}".fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-p8h3Sl/YRByZfZTAKXdsvF6xEenXKrXSVvpphmZENH4=";
          };

          rustToolchain = fenix.packages."${system}".combine [
            fenix.packages."${system}".latest.rustfmt
            toolchainFile
          ];
        in
        {
          default = pkgs.mkShell {
            buildInputs = [
              rustToolchain
            ];
          };
        }
      );
    };
}
