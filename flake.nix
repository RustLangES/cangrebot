{
  nixConfig = {
    extra-substituters = [
      "https://cache.rustlang-es.org/main"
    ];
    extra-trusted-public-keys = [
      "main:NnVmqBjdfyyL4tGgoTw17lUMDgulJ75+67pOsJupnS4="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }@inputs:
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        appBundle = pkgs.callPackage ./. { inherit inputs; };
      in {
        inherit (appBundle) apps packages devShells;
    });
}
