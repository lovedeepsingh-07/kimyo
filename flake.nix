{
  description = "kimyo";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/d2ed99647a4b195f0bcc440f76edfa10aeb3b743";
    flake-utils.url =
      "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };
  outputs = { ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import inputs.rust-overlay)
        ];
        pkgs = import inputs.nixpkgs {
          inherit system overlays;
        };
        rust-pkg = pkgs.rust-bin.stable."1.88.0".default;
        crane-lib = (inputs.crane.mkLib pkgs).overrideToolchain rust-pkg;
        lua-pkg = (import ./lua.nix { inherit system pkgs; }).pkg;
      in
      rec {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.just
            rust-pkg
            lua-pkg
          ];
        };
        packages.default =
          crane-lib.buildPackage {
            src = crane-lib.cleanCargoSource ./.;
            strictDeps = true;
          };
        apps.default = {
          type = "app";
          program = "${packages.default}/bin/kimyo";
        };
      });
}
