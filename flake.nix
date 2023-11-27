{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ cmake ]
            ++ lib.optionals pkgs.stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [ Carbon Cocoa Kernel ]);
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ]
            ++ lib.optionals pkgs.stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [ Carbon Cocoa Kernel libiconv ]);
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        formatter = nixpkgs.legacyPackages.${system}.nixpkgs-fmt;
      });
}
