{
    description = "Rocket.rs hosted server for 3DS SaveSync";
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11"; 
    };

    outputs = { self, nixpkgs, systems }: let
        forEachSystem = nixpkgs.lib.genAttrs (import systems); 
    in {
        devShells = forEachSystem (system: let
            pkgs = nixpkgs.legacyPackages.${system};
        in {
            default = pkgs.mkShell rec {
                packages = with pkgs; [
                    rustc
                    cargo
                ];

                nativeBuildInputs = with pkgs; [
                    gcc
                    pkg-config
                    redocly
                    act
                ];

                buildInputs = with pkgs; [
                    gtk3
                    libappindicator-gtk3
                    xdotool
                ];

                LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
                RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };
        });


        packages = forEachSystem (system: let
            pkgs = nixpkgs.legacyPackages.${system};
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in {
            default = pkgs.rustPlatform.buildRustPackage  {
                pname = manifest.name;
                version = manifest.version;
                
                cargoLock.lockFile = ./Cargo.lock;
                src = pkgs.lib.cleanSource ./.;

                nativeBuildInputs = with pkgs; [
                    pkg-config
                ];

                buildInputs = with pkgs; [
                    gtk3
                    libappindicator-gtk3
                    xdotool
                ];

                meta = with pkgs.lib; {
                    description = "Server for my 3DS Save Sync program";
                    homepage = "https://github.com/coolguy1842/SaveSyncd/";
                    license = licenses.gpl3;
                };
            };
        });
    };
}