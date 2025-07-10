{
  description = "kimyo";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/d98abf5cf5914e5e4e9d57205e3af55ca90ffc1d";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        formatter = pkgs.nixfmt-classic;
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.just
            pkgs.cmake
            pkgs.gcc
            pkgs.rust-bin.stable."1.84.0".default
          ];
        };
      });
}
