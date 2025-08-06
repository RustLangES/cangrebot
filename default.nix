{
  pkgs ? import <nixpkgs> { },
  inputs,
  ...
}: let
  cargoManifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

  buildInputs = with pkgs; [
    pkg-config
    libopus
    cmake
  ];

  commonArgs = rec{
    name = pname;
    pname = cargoManifest.package.name;
    version = cargoManifest.package.version;
    doCheck = false;
    src = pkgs.lib.cleanSourceWith {
      src = ./.;
      filter = path: type:
        (baseNameOf path) == "Cargo.lock"
        || (baseNameOf path) == ".gitmodules"
        || (pkgs.lib.hasSuffix ".rs" path)
        || (pkgs.lib.hasSuffix ".toml" path)
        || (pkgs.lib.hasInfix "crates" path)
        || (pkgs.lib.hasInfix "src" path)
        || (pkgs.lib.hasInfix "static/rust-examples" path)
        || (pkgs.lib.hasInfix "static" path);
    };

    inherit buildInputs;
  };

  appPkg = craneLib.buildPackage (commonArgs // {
    cargoArtifacts = craneLib.buildDepsOnly commonArgs;
  });

  containerPkg = let
    staticPkg = pkgs.stdenv.mkDerivation {
      name = "static-content";
      src = ./static;
      phases = [ "installPhase" ];
      installPhase = ''
        mkdir -p $out/app_static
        cp -r $src/* $out/app_static/
      '';
    };
  in pkgs.dockerTools.buildLayeredImage rec {
    name = cargoManifest.package.name;
    tag = cargoManifest.package.version;
    created = "now";

    contents = [ appPkg staticPkg ];
    config.Cmd = ["/bin/${name}"];
  };
in {
  # `nix run`
  apps.default = appPkg;

  # `nix build`
  packages = {
    default = appPkg;
    image = containerPkg;
  };

  # `nix develop`
  devShells.default = pkgs.mkShell {
    packages = [ toolchain ] ++ buildInputs;
  };
}
