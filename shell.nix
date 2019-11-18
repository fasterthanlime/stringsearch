{ pkgs ? import <nixpkgs> {} }:

let
  moz_overlay = import (builtins.fetchTarball "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  custom = import (builtins.fetchTarball {
    name = "killercup-nixpkgs";
    url = "https://github.com/killercup/nixpkgs/archive/cargo-fuzz-0.5.4.tar.gz";
  }) {};
in pkgs.mkShell {
  buildInputs = with pkgs; [
    git
    (nixpkgs.rustChannelOf { date = "2019-10-23"; channel = "nightly"; }).rust
    custom.pkgs.cargo-fuzz
  ];

  RUSTFLAGS="-C link-arg=-fuse-ld=gold";
  RUST_BACKTRACE = 1;
}

