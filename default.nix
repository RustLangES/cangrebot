{
  pkgs ? import <nixpkgs> { },
  ...
}: let
  toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
  cargoManifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);

  buildInputs = with pkgs; [
    pkg-config
    openssl
    libopus
  ];

  appPkg = pkgs.rustPlatform.buildRustPackage {
    pname = cargoManifest.package.name;
    version = cargoManifest.package.version;
    src = pkgs.lib.cleanSourceWith {
      src = ./.;
      filter = path: type:
        (baseNameOf path) == "Cargo.lock"
        || (pkgs.lib.hasSuffix ".rs" path)
        || (pkgs.lib.hasSuffix ".toml" path)
        || (pkgs.lib.hasInfix "crates" path)
        || (pkgs.lib.hasInfix "src" path)
        || (pkgs.lib.hasInfix "static/rust-examples" path)
        || (pkgs.lib.hasInfix "static" path);
    };
    cargoLock.lockFile = ./Cargo.lock;

    inherit buildInputs;

    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";
  };
in {
  # `nix run`
  apps.default = appPkg;
  # `nix build`
  packages.default = appPkg;
  # `nix develop`
  devShells.default = pkgs.mkShell {
    packages = [ toolchain ] ++ buildInputs;
  };
}
