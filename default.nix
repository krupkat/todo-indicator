{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ rustc cargo gcc rustfmt clippy just pkg-config openssl.dev libxkbcommon.dev libxkbcommon.dev ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}