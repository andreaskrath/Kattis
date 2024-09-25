{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    go-pin.url = "github:NixOS/nixpkgs/932fc16b263f26803d3960e4400bc13dde84a972";
    haskell-pin.url = "github:NixOS/nixpkgs/a71323f68d4377d12c04a5410e214495ec598d4c";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    go-pin,
    haskell-pin,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        go-pkgs = import go-pin {
          inherit system;
        };
        haskell-pkgs = import haskell-pin {
          inherit system;
        };
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              rust-bin.stable."1.80.1".default
              rust-analyzer

              go-pkgs.go
              go-pkgs.gopls

              haskell-pkgs.ghc
              haskell-pkgs.haskell-language-server
              ormolu
            ];
          };
      }
    );
}
