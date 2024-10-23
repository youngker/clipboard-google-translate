{
  description = "clipboard-google-translate";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            pkg-config
            alsa-lib
            libudev-zero
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            libxkbcommon
            xorg.libxcb
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [ Carbon Cocoa Kernel libiconv ]);
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
              with pkgs;
              pkgs.lib.makeLibraryPath [
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                libxkbcommon
                xorg.libxcb
                pkgs.vulkan-loader
                pkgs.glfw
              ]
          }";
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        formatter = nixpkgs.legacyPackages.${system}.nixpkgs-fmt;
      }
    );
}
