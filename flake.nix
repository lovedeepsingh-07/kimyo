{
  description = "kimyo";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/ca77296380960cd497a765102eeb1356eb80fed0";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ ];
        pkgs = import nixpkgs { inherit system overlays; };
        deps = import ./deps.nix { inherit pkgs; };
      in {
        formatter = pkgs.nixfmt-classic;
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.just
            pkgs.cmake
            pkgs.gcc
          ];
        };
        apps.setup = flake-utils.lib.mkApp {
          drv = pkgs.writeShellApplication {
            name = "setup";
            runtimeInputs = [ ];
            text = deps.setup_script;
          };
        };
      });
}
