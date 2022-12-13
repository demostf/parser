{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "nixpkgs/release-22.11";
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
      naersk = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.tf-demo-parser = naersk.buildPackage {
        pname = "tf-demo-parser";
        root = ./.;
      };
      defaultPackage = packages.tf-demo-parser;

      # `nix run`
      apps.tf-demo-parser = utils.lib.mkApp {
        drv = packages.tf-demo-parser;
      };
      defaultApp = apps.tf-demo-parser;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [rustc cargo bacon cargo-edit cargo-outdated clippy cargo-audit hyperfine valgrind];
      };
    });
}
