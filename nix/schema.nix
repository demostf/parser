{
  stdenv,
  rustPlatform,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ../. ["Cargo.*" "(src|benches)(/.*)?"];
in
  rustPlatform.buildRustPackage {
    pname = "demostf-parser-schema";
    version = "0.1.0";

    cargoBuildFlags = ''
      --bin schema
    '';

    src = lib.traceVal src;

    buildType = "debug";
    buildFeatures = ["schema"];

    doCheck = false;

    cargoLock = {
      lockFile = ../Cargo.lock;
    };
  }
