{
  inputs = {
    nixpkgs.url = "nixpkgs/release-23.05";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:icewind1991/naersk?rev=21b870efb320d44ec1c2f661f6e6e8deca9bb239";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    rust-overlay,
  }:
    utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = (import nixpkgs) {
        inherit system overlays;
      };
      naersk' = pkgs.callPackage naersk {};
      hostTarget = pkgs.hostPlatform.config;
      targets = ["x86_64-unknown-linux-musl" hostTarget];
      lib = pkgs.lib;
      naerskForTarget = target: let
        toolchain = pkgs.rust-bin.stable.latest.default.override { targets = [target]; };
      in pkgs.callPackage naersk {
        cargo = toolchain;
        rustc = toolchain;
      };
      hostNaersk = naerskForTarget hostTarget;
      src = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "(src|benches|tests|test_data)(/.*)?"];
      nearskOpt = {
        pname = "dispenser";
        root = src;
      };
    in rec {
      packages = (lib.attrsets.genAttrs targets (target: (naerskForTarget target).buildPackage {
        pname = "tf-demo-parser";
        root = src;
      })) // rec {
        tf-demo-parser = packages.${hostTarget};
        check = hostNaersk.buildPackage (nearskOpt // {
          checkOnly = true;
        });
        clippy = hostNaersk.buildPackage (nearskOpt // {
          clippyOnly = true;
        });
        test = hostNaersk.buildPackage (nearskOpt // {
          release = false;
          testOnly = true;
        });
        default = tf-demo-parser;
      };

      apps = rec {
        tf-demo-parser = utils.lib.mkApp {
          drv = packages.tf-demo-parser;
          exePath = "/bin/parse_demo";
        };
        default = tf-demo-parser;
      };

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default bacon cargo-edit cargo-outdated rustfmt clippy cargo-audit hyperfine valgrind];
      };
    });
}
