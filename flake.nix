{
    description = "Wini flake";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        systems.url = "github:nix-systems/default";
        rust-overlay.url = "github:oxalica/rust-overlay";
        flake-utils.url  = "github:numtide/flake-utils";
    };

    outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
        flake-utils.lib.eachDefaultSystem (system:
        let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
                inherit system overlays;
            };
        in
        {
            devShells.default = with pkgs; mkShell {
                buildInputs = [
                    watchexec
                    yq-go
                    coreutils
                    bun
                    dart-sass
                    fd
                    ripgrep
                    rust-bin.nightly.latest.default
                    just
                ];
            };

            packages.default = pkgs.stdenv.mkDerivation {
                name = "build-server";
                buildInputs = with pkgs; [
                    yq-go
                    coreutils
                    bun
                    dart-sass
                    fd
                    ripgrep
                    rust-bin.nightly.latest.default
                    just
                ];
                installPhase = ''
                    just build-prod
                '';
            };
        }
    );
}
