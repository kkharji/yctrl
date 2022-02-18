{
  inputs = {
    naersk-lib.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";
  };
  outputs = { self, nixpkgs, utils, naersk-lib, devshell }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          system = system;
          overlays = [ devshell.overlay ];
        };
        naersk = pkgs.callPackage naersk-lib { };
      in {
        defaultPackage = naersk.buildPackage ./.;
        defaultApp = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };
        devShell = pkgs.devshell.mkShell {
          imports = [ (pkgs.devshell.importTOML ./devshell.toml) ];
          # env = [{ # FAILS with rust-anaylzer
          #   name = "RUST_SRC_PATH";
          #   value = "${pkgs.rustPlatform.rustLibSrc}";
          # }];
        };
      });
}
