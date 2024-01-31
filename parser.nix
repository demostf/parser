{
  stdenv,
  rustPlatform,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ./. ["Cargo.*" "(src|benches|tests|test_data)(/.*)?"];
in
  rustPlatform.buildRustPackage rec {
    pname = "demostf-parser";
    version = "0.1.0";

    inherit src;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };
  }
