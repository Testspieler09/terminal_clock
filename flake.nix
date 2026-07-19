{
  description = "A simple but fancy looking customizable terminal clock";

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs    = nixpkgs.legacyPackages.${system};
        package = pkgs.callPackage ./nix/package.nix { };
      in
      {
        packages.default = package;

        apps.default = {
          type    = "app";
          program = "${package}/bin/tc";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      }
    ) // {
      homeManagerModules.default = import ./nix/home-manager.nix;

      overlays.default = final: _prev: {
        terminal-clock = final.callPackage ./nix/package.nix { };
      };
    };
}
