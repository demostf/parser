{
  inputs = {
    nixpkgs.url = "nixpkgs/release-23.05";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
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
      src = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "(src|tests|benches)(/.*)?"];
    in rec {
      packages = (lib.attrsets.genAttrs targets (target: (naerskForTarget target).buildPackage {
        pname = "tf-demo-parser";
        root = src;
      })) // rec {
        tf-demo-parser = packages.${hostTarget};
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
