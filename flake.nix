{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.tf-demo-parser = naersk-lib.buildPackage {
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
        nativeBuildInputs = with pkgs; [rustc cargo bacon cargo-edit cargo-outdated clippy];
      };
    });
}
