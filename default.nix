{
  pkgs ? import <nixpkgs> { },
  system ? builtins.currentSystem,
  ...
}: let
  cargoManifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  systemToTarget = system: let
    arch = builtins.elemAt (pkgs.lib.splitString "-" system) 0;
    os = builtins.elemAt (pkgs.lib.splitString "-" system) 1;
  in
    if os == "darwin"
    then "${arch}-apple-darwin"
    else if os == "linux"
    then "${arch}-unknown-linux-gnu"
    else throw "Unsupported system: ${system}";

  hostTarget = systemToTarget system;
  toolchain = target:
    (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
      targets = [hostTarget target];
    };

  architectures = [
    { arch = "x86_64"; target = "x86_64-unknown-linux-gnu"; }
    # { arch = "aarch64"; target = "aarch64-unknown-linux-gnu"; }
  ];

  buildInputs = with pkgs; [
    pkg-config
    libopus
  ];

  appPkg = { arch, target, ... }: let
    target_name = pkgs.lib.toUpper (builtins.replaceStrings ["-"] ["_"] target);
  in pkgs.rustPlatform.buildRustPackage rec {
    pname = cargoManifest.package.name;
    name = "${pname}-${arch}";
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
    cargoLock.lockFile = ./Cargo.lock;

    inherit buildInputs;

    RUSTFLAGS = "-C relocation-model=static";
    TARGET_CC = "${pkgs.stdenv.cc.targetPrefix}cc";
    "CARGO_TARGET_${target_name}_LINKER" = "${pkgs.llvmPackages.lld}/bin/ld.lld";
    "CARGO_TARGET_${target_name}_RUNNER" = "qemu-${arch}";
  };

  containerPkg = variant: let
    pkg = appPkg variant;
    arch = if variant.arch == "x86_64" then "amd" else "arm";
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
    architecture = "linux/${arch}64";

    contents = [ pkg staticPkg ];
    config.Cmd = ["/bin/${name}"];
  };

  generatedMatrixJson = builtins.toJSON (pkgs.lib.flatten (map ({ arch, ... }:
    { arch = arch; }
  ) architectures));
in {
  # `nix run`
  apps = {
    default = appPkg;
    matrix = {
      type = "app";
      program = toString (pkgs.writeScript "generate-matrix" ''
        #!/bin/sh
        echo '${generatedMatrixJson}'
      '');
    };
  };

  # `nix build`
  packages = {
    default = appPkg;
  } // (pkgs.lib.listToAttrs (map ({arch, ...} @ args: {
    name = "image-${arch}";
    value = containerPkg args;
  }) architectures));

  # `nix develop`
  devShells.default = pkgs.mkShell {
    packages = [ toolchain ] ++ buildInputs;
  };
}
