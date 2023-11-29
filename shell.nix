{ pkgs ? import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; } }:
let
  rust = pkgs.rust-bin.stable."1.66.1".default.override {
    extensions = [ "rust-src" ];
  };
  go_pin = import (pkgs.fetchFromGitHub{
		owner = "nixos";
		repo = "nixpkgs";
		rev = "932fc16b263f26803d3960e4400bc13dde84a972";
		sha256 = "sha256-f5XNR9i7ImhGHGys/0SpGX9OD2SFqWfNKlKtdn7c3Vc=";
	}) {}; 
in
pkgs.mkShell {
  nativeBuildInputs = [
    go_pin.go
    rust
  ];

  RUST_BACKTRACE = "1";
}
