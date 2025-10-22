{
  description = "kimyo";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/ca77296380960cd497a765102eeb1356eb80fed0";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
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
        lua-pkg = (import ./lua.nix { inherit system pkgs; }).pkg;
      in
      {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.just
            rust-pkg
            lua-pkg
          ];
        };
      });
}
