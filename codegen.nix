{
  stdenv,
  rustPlatform,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ./. ["Cargo.*" "(src|benches|tests|test_data)(/.*)?"];
in
  rustPlatform.buildRustPackage {
    pname = "demostf-parser-codegen";
    version = "0.1.0";

    src = lib.traceVal src;

    buildType = "debug";
    buildFeatures = ["codegen"];

    doCheck = false;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };
  }
