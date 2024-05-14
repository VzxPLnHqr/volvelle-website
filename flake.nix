{
  # example taken from https://github.com/tomhoule/rust-nix-wasm32-unknown-unknown-example/blob/main/flake.nix
  description = "Minimal rust wasm32-unknown-unknown example";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./volvelle-wasm/rust-toolchain.toml;
        inputs = [ rust pkgs.wasm-bindgen-cli ];
      in
      {
        packages.codex32-website = pkgs.rustPlatform.buildRustPackage {
          pname = "codex32-website";
          version = "1.0.0";

          src = ./volvelle-wasm/.;

          cargoLock = {
            lockFile = ./volvelle-wasm/Cargo.lock;
          };

          nativeBuildInputs = inputs;

          buildPhase = ''
            echo 'Creating out dir...'
            mkdir -p $out/target;
            mkdir -p $out/www/pkg;
            
            cargo build --release --target=wasm32-unknown-unknown --target-dir=$out/target

            # Optional, of course
            # echo 'Copying package.json...'
            # cp ./package.json $out/;

            echo 'Generating node module...'
            wasm-bindgen \
              --target no-modules \
              --out-dir $out/www/pkg \
              $out/target/wasm32-unknown-unknown/release/volvelle_wasm.wasm;

            cp -a ${./www}/. $out/www
          '';
          installPhase = "echo 'Skipping installPhase'";
        };

        defaultPackage = self.packages.x86_64-linux.codex32-website;

        devShell = pkgs.mkShell { packages = inputs; };
      }
    );
}