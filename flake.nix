{
  description = "Tidy trees";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;

    flake-parts.url = "github:hercules-ci/flake-parts";

    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";

    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    import-cargo.url = github:edolstra/import-cargo;

    flake-compat.url = github:edolstra/flake-compat;
    flake-compat.flake = false;
  };

  outputs = { self, flake-parts, ... }@inputs:
    let
      # Keep in sync with the version in package.json.
      version = "0.0.2";

      wasmSrc = ./rust;

      overlays = [
        (
          final: prev: {
            # https://github.com/tweag/nickel/blob/d3f7192c4e800860dace6e22c14a91610657a675/flake.nix#L46
            #
            # The version of `wasm-bindgen` CLI *must* be the same as
            # the `wasm-bindgen` Rust dependency in `Cargo.toml`, or else the build will fail:
            #
            # https://github.com/rustwasm/wasm-pack/issues/1138

            # The definition of `wasm-bindgen-cli` in Nixpkgs does not
            # allow overriding directly the attrset passed to
            # `buildRustPackage`. We instead override the attrset that
            # `buildRustPackage` generates and passes to
            # `mkDerivation`. See
            # https://discourse.nixos.org/t/is-it-possible-to-override-cargosha256-in-buildrustpackage/4393
            wasm-bindgen-cli = prev.wasm-bindgen-cli.overrideAttrs (oldAttrs:
              let
                # Note: this package's Cargo lockfiles specify several
                # versions of `wasm-bindgen`, but this appears to be
                # the one the build process wants.
                wasmBindgenCargoVersion = "0.2.80";
              in
              rec {
                pname = "wasm-bindgen-cli";
                version = wasmBindgenCargoVersion;

                src = final.fetchCrate {
                  inherit pname version;
                  sha256 = "sha256-f3XRVuK892TE6xP7eq3aKpl9d3fnOFxLh+/K59iWPAg=";
                };

                cargoDeps = oldAttrs.cargoDeps.overrideAttrs (final.lib.const {
                  inherit src;
                  outputHash = "sha256-9RAAyHHBDzqaR8oE8ggX4X8t/zIJUmj8D1phOOh7Cd0=";
                });
              });
          }
        )
      ];

      # Filter out source that shouldn't cause a Nix rebuild.
      customFilter = src:
        let
          srcIgnored = inputs.gitignore.lib.gitignoreFilterWith {
            basePath = src;
            extraRules = ''
              *.nix
            '';
          };
        in
        path: type:
          srcIgnored path type;
    in
    flake-parts.lib.mkFlake { inherit self; } {
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          # Note that the flake's own overlays aren't included in the
          # `pkgs` argument to `perSystem`, so we have to override it
          # ourselves. :\
          #
          # https://github.com/hercules-ci/flake-parts/issues/69#issuecomment-1328290823
          overlayPkgs = import inputs.nixpkgs {
            inherit system;
            inherit overlays;
          };

          rust-wasm-toolchain = with inputs.fenix.packages.${system};
            combine [
              minimal.rustc
              minimal.cargo
              targets.wasm32-unknown-unknown.latest.rust-std
            ];

          # Via:
          # https://github.com/tweag/nickel/blob/d3f7192c4e800860dace6e22c14a91610657a675/flake.nix#L84
          cargoHome = (inputs.import-cargo.builders.importCargo {
            lockFile = ./rust/Cargo.lock;
            inherit pkgs;
          }).cargoHome;

          tidy-wasm-pkgs = pkgs.stdenv.mkDerivation {
            pname = "tidy-wasm-pkgs";
            inherit version;

            src = wasmSrc;

            buildInputs = with pkgs; [
              wasm-pack
              wasm-bindgen-cli
              binaryen
            ] ++ [
              rust-wasm-toolchain
              cargoHome
            ];

            buildPhase = ''
              cd crates/wasm
              wasm-pack build --mode no-install --target web --release -- --frozen --offline
            '';

            installPhase = ''
              mkdir -p $out
              cp -r pkg $out/wasm_dist
            '';
          };

          tidy = pkgs.buildNpmPackage {
            pname = "tidy";
            inherit version;

            src = pkgs.lib.cleanSourceWith {
              filter = customFilter ./.;
              src = ./.;
              name = "tidy-source";
            };

            npmDepsHash = "sha256-rE8XdEqEzWSjnpdOGPbILTJJubnrK/v80f/iZBhIqB8=";
            npmFlags = [ "--legacy-peer-deps" ];

            buildInputs = [
              tidy-wasm-pkgs
            ];

            configurePhase = ''
              ln -s ${tidy-wasm-pkgs}/wasm_dist .
            '';

            doCheck = true;

            checkPhase = ''
              npm run test
              npm audit --omit dev
            '';

            buildPhase = ''
              ./node_modules/.bin/vite build
            '';
          };

        in
        {
          # See comment above re: `pkgs` and overlays.
          _module.args.pkgs = overlayPkgs;

          packages = {
            inherit tidy-wasm-pkgs tidy;
          };

          # Note that this shell does not attempt to provide the same
          # package set as the Nix build. It simply uses Nix to
          # install the tools needed to build the package using
          # standard `npm` commands.
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              wasm-pack
              wasm-bindgen-cli
              nodejs-18_x
            ] ++ [
              rust-wasm-toolchain
            ];

            shellHook = ''
              npm install
            '';
          };
        };

      flake =
        let
          pkgs = inputs.nixpkgs.legacyPackages."x86_64-linux";
          recurseIntoHydraJobs = set:
            let
              scrubForNix = name: builtins.replaceStrings [ ":" ] [ "-" ] name;
              recurse = path: set:
                let
                  g =
                    name: value: pkgs.lib.nameValuePair (scrubForNix name) (
                      if pkgs.lib.isAttrs value
                      then ((recurse (path ++ [ name ]) value) // { recurseForDerivations = true; })
                      else value
                    );
                in
                pkgs.lib.mapAttrs' g set;
            in
            recurse [ ] set;
        in
        {
          hydraJobs = {
            inherit (self) packages;
          }
          // {
            required = pkgs.releaseTools.aggregate {
              name = "required";
              constituents = builtins.map builtins.attrValues (with self.hydraJobs; [
                packages.x86_64-linux
                packages.aarch64-darwin
              ]);
              meta.description = "Required CI builds";
            };
          };

          ciJobs = recurseIntoHydraJobs self.hydraJobs;
        };
    };
}
