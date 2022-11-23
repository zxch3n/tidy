{
  description = "Tidy trees";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;

    flake-parts.url = "github:hercules-ci/flake-parts";

    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";

    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    flake-compat.url = github:edolstra/flake-compat;
    flake-compat.flake = false;
  };

  outputs = { self, flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit self; } {
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          rust-wasm-toolchain = with inputs.fenix.packages.${system};
            combine [
              minimal.rustc
              minimal.cargo
              targets.wasm32-unknown-unknown.latest.rust-std
            ];
        in
        {
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              wasm-pack
              nodejs-18_x
            ] ++ [
              rust-wasm-toolchain
            ];

            shellHook = ''
              npm install
            '';
          };
        };

      flake = { };
    };
}
