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

    cargoBuildFlags = ''
      --bin parse_demo
    '';

    inherit src;

    doCheck = stdenv.system == "x86_64-linux"; # building the tests takes +- forever on aarch64 for some reason

    cargoLock = {
      lockFile = ./Cargo.lock;
      outputHashes = {
        "schemars-0.8.16" = "sha256-mQR56Ym76gSRulZrThmZHHw2JfhEgYhWXabwaYmyMYs=";
      };
    };
  }
