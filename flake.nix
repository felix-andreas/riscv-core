{
  inputs = {
    # base
    systems.url = "github:nix-systems/default";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    # extra
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
      # see: https://github.com/NixOS/nix/issues/5790
      inputs.flake-utils.inputs.systems.follows = "systems";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      # see: https://github.com/NixOS/nix/issues/5790
      inputs.flake-utils.inputs.systems.follows = "systems";
    };
  };

  outputs =
    { self
      # base
    , systems
    , nixpkgs
      # extra
    , crane
    , devshell
    , rust-overlay
    } @ inputs:
    let
      l = inputs.nixpkgs.lib // builtins;
      fs = l.fileset;
      eachSystem = fn: l.genAttrs (import inputs.systems) fn;
      flake = (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              inputs.devshell.overlays.default
              (import inputs.rust-overlay)
            ];
          };
          pkgsRiscv = pkgs.pkgsCross.riscv32-embedded.buildPackages;
          rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            });
          craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;
          rustFiles = fs.fileFilter (file: file.hasExt "rs") ./.;
          webFiles = fs.fileFilter (file: l.any file.hasExt [ "html" "css" "js" ]) ./.;
          cargoFiles = fs.unions [
            (fs.fileFilter (file: file.name == "Cargo.toml" || file.name == "Cargo.lock") ./.)
          ];
          commonArgs = {
            pname = "crate";
            version = "0.1";
            strictDeps = true;
            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          };
          crateDepsOnly = craneLib.buildDepsOnly (commonArgs // {
            cargoCheckCommandcargo = "check --profile release --all-targets --all-features";
            src = fs.toSource {
              root = ./.;
              fileset = cargoFiles;
            };
          });
          crateClippy = craneLib.cargoClippy (commonArgs // {
            cargoArtifacts = crateDepsOnly;
            cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
            src = fs.toSource {
              root = ./.;
              fileset = fs.unions ([
                cargoFiles
                rustFiles
              ]);
            };
          });
        in
        rec {
          devShell = pkgs.devshell.mkShell {
            motd = "";
            packages = with pkgs; [
              # Rust
              bacon
              cargo-expand
              cargo-sort
              evcxr
              rust-toolchain
              # Leptos
              leptosfmt
              trunk
              dart-sass
              binaryen
              nodePackages.tailwindcss
              # RISC-V
              gnumake
              autoconf
              pkgsRiscv.gcc
              pkgsRiscv.binutils
              # Python
              (python311.withPackages (p: with p; [ black httpx ipykernel ipython isort matplotlib numpy pytorch tqdm transformers ]))
            ];

            commands = [
              {
                name = "riscv64-unknown-elf-gcc";
                command = ''
                  ${pkgsRiscv.gcc}/bin/riscv64-none-elf-gcc "$@"
                '';
              }
              {
                name = "riscv64-unknown-elf-objdump";
                command = ''
                  ${pkgsRiscv.binutils}/bin/riscv64-none-elf-objdump "$@"
                '';
              }
            ];
          };
          check = crateClippy;
          package = craneLib.buildTrunkPackage (commonArgs // {
            pname = "web";
            cargoArtifacts = crateClippy;
            cargoExtraArgs = "--package=client";
            trunkIndexPath = "web/index.html";
            src = fs.toSource {
              root = ./.;
              fileset = fs.unions ([
                cargoFiles
                rustFiles
                webFiles
              ]);
            };
            nativeBuildInputs = with pkgs; [ nodePackages.tailwindcss ];

            # The version of wasm-bindgen-cli here must match the one from Cargo.lock.
            wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
              version = "0.2.92";
              hash = "sha256-1VwY8vQy7soKEgbki4LD+v259751kKxSxmo/gqE6yV0=";
              cargoHash = "sha256-aACJ+lYNEU8FFBs158G1/JG8sc6Rq080PeKCMnwdpH0=";
            };
          });
          publish = with pkgs; writeShellApplication {
            name = "publish";
            runtimeInputs = [ wrangler ];
            text = ''
              wrangler pages deploy --project-name=riscv-felixandreas ${package}
            '';
          };
        });
    in
    {
      checks = eachSystem (system: { default = (flake system).check; });
      devShells = eachSystem (system: { default = (flake system).devShell; });
      packages = eachSystem (system: {
        default = (flake system).package;
        publish = (flake system).publish;
      });
    };
}
