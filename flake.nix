{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = {self, nixpkgs, ...}: let
    systems = ["x86_64-linux" "aarch64-linux"];
    forAllSystems = nixpkgs.lib.genAttrs systems;
    pkgsFor = system: import nixpkgs {
      inherit system;
    };
  in {
      packages = forAllSystems (system: let
      pkgs = pkgsFor system;
    in {
      devnotify = pkgs.rustPlatform.buildRustPackage {
        pname = "devnotify";
        version = "0.1.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustc
          cargo
        ];

        buildInputs = with pkgs; [
          udev
          openal
          libsndfile
          libnotify
          glib
        ];

        meta = with pkgs.lib; {
          license = licenses.mit;
          platforms = platforms.linux;
          maintainers = with maintainers; [ ShyAssassin ];
          homepage = "https://github.com/ShyAssassin/DevNotify";
          description = "A simple tool to notify you when you plug in a usb device";
        };
      };
    });

    devShells = forAllSystems (system: let
      pkgs = pkgsFor system;
    in {
      default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          rustup mold
          pkg-config udev
          openal libsndfile
          libnotify glib
        ];
        shellHook = ''
          rustup default 1.85.1
          rustup component add rust-src rust-std
          rustup component add rust-docs rust-analyzer
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
          export RUSTFLAGS="$RUSTFLAGS -C linker=${pkgs.clang}/bin/clang -C link-arg=-fuse-ld=${pkgs.mold}/bin/mold"
        '';
      };
    });
  };
}
