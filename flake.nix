{
  inputs = {
    nixpkgs.url = "nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
    cross-naersk.url = "github:icewind1991/cross-naersk";
    cross-naersk.inputs.nixpkgs.follows = "nixpkgs";
    cross-naersk.inputs.naersk.follows = "naersk";
  };

  outputs = {
    self,
    flake-utils,
    cross-naersk,
    naersk,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    inherit (flake-utils.lib) eachDefaultSystem eachSystem;
  in (eachDefaultSystem (system: let
      overlays = [
        (import rust-overlay)
        (import ./overlay.nix)
      ];
      pkgs = (import nixpkgs) {
        inherit system overlays;
      };
      lib = pkgs.lib;

      hostTarget = pkgs.hostPlatform.config;
      targets = [
        "x86_64-unknown-linux-musl"
        "i686-unknown-linux-musl"
        "armv7-unknown-linux-musleabihf"
        "aarch64-unknown-linux-musl"
        "x86_64-pc-windows-gnu"
        hostTarget
      ];

      releaseTargets = lib.lists.remove hostTarget targets;

      artifactForTarget = target: "parse_demo${cross-naersk'.execSufficForTarget target}";
      assetNameForTarget = target: "parser-${builtins.replaceStrings ["-unknown" "-gnu" "-musl" "eabihf" "-pc"] ["" "" "" "" ""] target}${cross-naersk'.execSufficForTarget target}";

      cross-naersk' = pkgs.callPackage cross-naersk {inherit naersk;};
      nearskOpt = {
        inherit (pkgs.demostf-parser) pname src;
      };

      buildMatrix = targets: {
        include =
          builtins.map (target: {
            inherit target;
            artifact_name = artifactForTarget target;
            asset_name = assetNameForTarget target;
          })
          targets;
      };
      hostNaersk = cross-naersk'.hostNaersk;

      mkHydraJobs = system: {
        parser = derivation {
          name = "parser";
          builder = "mybuilder";
          inherit system;
        };
        nested = {
          attribute = derivation {
            name = "nested-attribute";
            builder = "mybuilder";
            inherit system;
          };
        };
      };
    in rec {
      packages =
        lib.attrsets.genAttrs targets (target:
          (cross-naersk'.buildPackage target) (nearskOpt
            // {
              overrideMain = args:
                args
                // {
                  preConfigure = ''
                    cargo_build_options="$cargo_build_options --bin parse_demo"
                  '';
                };
            }))
        // rec {
          inherit (pkgs) demostf-parser demostf-parser-codegen demostf-parser-codegen-events demostf-parser-codegen-props;
          check = hostNaersk.buildPackage (nearskOpt
            // {
              mode = "check";
            });
          clippy = hostNaersk.buildPackage (nearskOpt
            // {
              mode = "clippy";
            });
          test = hostNaersk.buildPackage (nearskOpt
            // {
              release = false;
              mode = "test";
            });
          default = demostf-parser;
        };

      inherit targets;
      inherit releaseTargets;
      matrix = buildMatrix targets;
      releaseMatrix = buildMatrix releaseTargets;

      apps = rec {
        tf-demo-parser = flake-utils.lib.mkApp {
          drv = packages.tf-demo-parser;
          exePath = "/bin/parse_demo";
        };
        default = tf-demo-parser;
      };

      checks = {
        fmt-check = pkgs.stdenvNoCC.mkDerivation {
          name = "fmt-check";
          src = ./.;
          doCheck = true;
          dontBuild = true;
          nativeBuildInputs = with pkgs; [alejandra shellcheck shfmt];
          checkPhase = ''
            alejandra -c .
          '';
          installPhase = ''
            mkdir $out
          '';
        };
      };

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default bacon cargo-edit cargo-outdated rustfmt clippy cargo-audit hyperfine valgrind cargo-insta cargo-semver-checks];
      };
    })
    // {
      overlays.default = import ./overlay.nix;
      hydraJobs = eachSystem ["x86_64-linux" "aarch64-linux"] (system: {
        parser = self.packages.${system}.demostf-parser;
      });
    });
}
