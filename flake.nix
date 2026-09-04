{
    description = "Wini flake";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        systems.url = "github:nix-systems/default";
        rust-overlay.url = "github:oxalica/rust-overlay";
        flake-utils.url  = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
        flake-utils.lib.eachDefaultSystem (system:
        let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
                inherit system overlays;
            };

            toolchain = pkgs.rust-bin.selectLatestNightlyWith (t: t.default);
            rustPlatform = pkgs.makeRustPlatform {
                cargo = toolchain;
                rustc = toolchain;
            };

            # JS deps normally installed by `bun i` + synced by
            # ./scripts/sync-packages.sh (cf. ./packages-files.toml).
            # Fetched directly so the build works in the Nix sandbox.
            wasmoonTarball = pkgs.fetchurl {
                url = "https://registry.npmjs.org/wasmoon/-/wasmoon-1.16.0.tgz";
                hash = "sha256-egfLbTmvEJEfq9UHQIM0d7GLR7bwMUjxUhjt9HTIr1Y=";
            };
            htmxTarball = pkgs.fetchurl {
                url = "https://registry.npmjs.org/htmx.org/-/htmx.org-1.9.12.tgz";
                hash = "sha256-P42xkDIjwknCDhiwnRvCaHduWchRQk9SUoXQuT2CXNA=";
            };

            datastarJs = pkgs.fetchurl {
                url = "https://raw.githubusercontent.com/starfederation/datastar/73ab00e7c06d8c2bad030fdddafba800fcccbde2/bundles/datastar.js";
                hash = "sha256-XWt3lKUKg9gtqWKuxeOC9a6DrHr7x1H5A/epxr1DPGU=";
            };
        in
        {
            devShells.default = with pkgs; mkShell {
                buildInputs = [
                    yq-go
                    coreutils
                    bun
                    gnused
                    git
                    iproute2
                    dart-sass
                    fd
                    ripgrep
                    just
                    toolchain
                    delta
                    taplo
                    watchexec
                ];
            };

            packages.default = rustPlatform.buildRustPackage {
                pname = "tuto-tk-run";
                version = "0.1.0";

                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;

                nativeBuildInputs = with pkgs; [
                    bun
                    dart-sass
                    fd
                    gnused
                    makeWrapper
                ];

                # Compile SCSS/TS and sync JS modules, like `just build-prod`,
                # but without touching the network.
                preBuild = ''
                    export HOME="$TMPDIR"
                    patchShebangs scripts

                    ./scripts/scss.sh
                    ./scripts/typescript.sh

                    mkdir -p public/modules/wasmoon public/modules/htmx.org
                    tar -xzf ${wasmoonTarball} --strip-components=2 \
                        -C public/modules/wasmoon package/dist/index.js
                    tar -xzf ${htmxTarball} --strip-components=2 \
                        -C public/modules/htmx.org package/dist/htmx.min.js

                    cp ${datastarJs} public/datastar.js
                '';

                # Tests expect a running environment; skip in the sandbox.
                doCheck = false;

                # The server resolves everything relative to its CWD:
                # ./wini.toml, ./packages-files.toml, .env, ./public,
                # ./src/pages/** (markdown content + compiled css/js).
                # Ship a runtime tree and wrap the binary to chdir into it.
                postInstall = ''
                    runtime="$out/share/tuto.tk.run"
                    mkdir -p "$runtime"
                    cp -r src public wini.toml packages-files.toml "$runtime/"

                    # dotenvy exits if no .env is found; real config comes
                    # from the environment (systemd), which takes precedence.
                    cat > "$runtime/.env" <<'EOF'
                    PORT=3000
                    ENV_TYPE="PROD"
                    EOF
                    sed -i 's/^ *//' "$runtime/.env"

                    mv "$out/bin/programming_tutorial" "$out/bin/.tuto-tk-run-unwrapped"
                    makeWrapper "$out/bin/.tuto-tk-run-unwrapped" "$out/bin/tuto-tk-run" \
                        --chdir "$runtime"
                '';

                meta.mainProgram = "tuto-tk-run";
            };
        })
        // {
            nixosModules.default = { config, lib, pkgs, ... }:
            let
                cfg = config.services.tuto-tk-run;
            in
            {
                options.services.tuto-tk-run = {
                    enable = lib.mkEnableOption "tuto.tk.run web server";

                    package = lib.mkOption {
                        type = lib.types.package;
                        default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
                        description = "The tuto.tk.run package to run.";
                    };

                    port = lib.mkOption {
                        type = lib.types.port;
                        default = 3000;
                        description = "Port the server listens on.";
                    };

                    envType = lib.mkOption {
                        type = lib.types.enum [ "PROD" "STAGING" "LOCAL" ];
                        default = "PROD";
                        description = "Value of ENV_TYPE (selects cache rules etc.).";
                    };

                    openFirewall = lib.mkOption {
                        type = lib.types.bool;
                        default = false;
                        description = "Open the port in the firewall.";
                    };
                };

                config = lib.mkIf cfg.enable {
                    systemd.services.tuto-tk-run = {
                        description = "tuto.tk.run web server";
                        wantedBy = [ "multi-user.target" ];
                        after = [ "network.target" ];

                        environment = {
                            PORT = toString cfg.port;
                            ENV_TYPE = cfg.envType;
                        };

                        serviceConfig = {
                            ExecStart = lib.getExe cfg.package;
                            Restart = "on-failure";
                            RestartSec = 5;

                            DynamicUser = true;
                            NoNewPrivileges = true;
                            PrivateTmp = true;
                            PrivateDevices = true;
                            ProtectSystem = "strict";
                            ProtectHome = true;
                            ProtectKernelTunables = true;
                            ProtectKernelModules = true;
                            ProtectControlGroups = true;
                            RestrictAddressFamilies = [ "AF_INET" "AF_INET6" ];
                            RestrictNamespaces = true;
                            LockPersonality = true;
                            MemoryDenyWriteExecute = false; # wasm/jit safety margin
                            SystemCallArchitectures = "native";
                            CapabilityBoundingSet = "";
                        };
                    };

                    networking.firewall.allowedTCPPorts =
                        lib.mkIf cfg.openFirewall [ cfg.port ];
                };
            };
        };
}

