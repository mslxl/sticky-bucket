{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
          (import ./nix/overlays)
        ];
      };

      rust-toolchain = pkgs.fenix.complete;

      libraries = with pkgs; [
        webkitgtk_4_1
        libsoup_3
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        openssl
        librsvg
        libayatana-appindicator
        (opencv.override (old: {
          enableGtk3 = true;
          enableFfmpeg = true;
        }))
        llvmPackages.libclang.lib
      ];

      buildInputs = libraries ++ (with pkgs; [
        just

        # Tauri
        nodejs
        nodejs.pkgs.pnpm
        (with rust-toolchain; [
          cargo
          rustc
          rust-src
          clippy
          rustfmt
        ])

        curl
        wget
        (with llvmPackages; [
          llvm
          libclang.lib
          clang
        ])
        pkg-config

        # Algorithm impl in CXX
        xmake
        cmake

        # OpenCV algorithm test
        (python3.withPackages (ps:
          with ps; [
            hy
            numpy
            (opencv4.override {enableGtk2 = true;})
          ]))
      ]);
    in rec {
      # Executed by `nix build`
      packages = rec {
        stickerbucket = pkgs.callPackage ./stickerbucket.nix {
          inherit libraries;
        };
        default = stickerbucket;
      };

      # Executed by `nix run`
      apps = rec {
        stickerbucket = {
          type = "app";
          program = "${packages.default}/bin/stickerbucket";
        };
        default = stickerbucket;
      };

      devShell = pkgs.mkShell {
        inherit buildInputs;

        shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
          export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
        '';

        WEBKIT_DISABLE_COMPOSITING_MODE = "1";
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        # Specify the rust-src path (many editors rely on this)
        RUST_SRC_PATH = "${rust-toolchain.rust-src}/lib/rustlib/src/rust/library";
      };

      formatter = nixpkgs.legacyPackages.${system}.alejandra;
    });
}
