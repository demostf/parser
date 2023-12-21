{
  inputs = {
    nixpkgs.url = "nixpkgs/release-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:icewind1991/naersk?rev=6d245a3bbb2ee31ec726bb57b9a8b206302e7110";
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
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
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
      src = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "(src|benches|tests|test_data)(/.*)?"];
      nearskOpt = {
        pname = "dispenser";
        root = src;
      };

      buildMatrix = targets: {
        include = builtins.map (target: {
          inherit target;
          artifact_name = artifactForTarget target;
          asset_name = assetNameForTarget target;
        }) targets;
      };
      hostNaersk = cross-naersk'.hostNaersk;
    in rec {
      packages = lib.attrsets.genAttrs targets (target: (cross-naersk'.buildPackage target) (nearskOpt // {
        overrideMain = args: args // {
          preConfigure = ''
            cargo_build_options="$cargo_build_options --bin parse_demo"
          '';
        };
      })) // rec {
        tf-demo-parser = packages.${hostTarget};
        check = hostNaersk.buildPackage (nearskOpt // {
          mode = "check";
        });
        clippy = hostNaersk.buildPackage (nearskOpt // {
          mode = "clippy";
        });
        test = hostNaersk.buildPackage (nearskOpt // {
          release = false;
          mode = "test";
        });
        default = tf-demo-parser;
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

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default bacon cargo-edit cargo-outdated rustfmt clippy cargo-audit hyperfine valgrind cargo-insta];
      };
    });
}
