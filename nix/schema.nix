{
  stdenv,
  rustPlatform,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
in
  rustPlatform.buildRustPackage {
    pname = "demostf-parser-schema";
    version = "0.1.0";

    cargoBuildFlags = ''
      --bin schema
    '';

    src = sourceByRegex ../. ["Cargo.*" "(src|benches)(/.*)?"];

    buildType = "debug";
    buildFeatures = ["schema"];

    doCheck = false;

    cargoLock = {
      lockFile = ../Cargo.lock;
    };

    meta.mainProgram = "schema";
  }
