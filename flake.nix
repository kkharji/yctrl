{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk-lib.url =
      "github:nix-community/naersk?rev=2fc8ce9d3c025d59fee349c1f80be9785049d653";
    utils.url =
      "github:numtide/flake-utils?rev=3cecb5b042f7f209c56ffd8371b2711a290ec797";
    devshell.url =
      "github:numtide/devshell?rev=7033f64dd9ef8d9d8644c5030c73913351d2b660";
    flake-compat = {
      url =
        "github:edolstra/flake-compat?rev=b7547d3eed6f32d06102ead8991ec52ab0a4f1a7";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk-lib, devshell, flake-compat }:
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
        overlay = f: p: { yctrl = self.defaultPackage."${system}"; };
        devShell = pkgs.devshell.mkShell {
          imports = [ (pkgs.devshell.importTOML ./devshell.toml) ];
          # env = [{ # FAILS with rust-anaylzer
          #   name = "RUST_SRC_PATH";
          #   value = "${pkgs.rustPlatform.rustLibSrc}";
          # }];
        };
      });
}
