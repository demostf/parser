{
  inputs = {
    nixpkgs.url = "nixpkgs/release-23.05";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = (import nixpkgs) {
        inherit system;
      };
      naersk' = pkgs.callPackage naersk {};
      lib = pkgs.lib;
      src = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "(src|tests|benches)(/.*)?"];
    in rec {
      packages = rec {
        tf-demo-parser = naersk'.buildPackage {
          pname = "tf-demo-parser";
          root = src;
        };
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
        nativeBuildInputs = with pkgs; [rustc cargo bacon cargo-edit cargo-outdated rustfmt clippy cargo-audit hyperfine valgrind];
      };
    });
}
