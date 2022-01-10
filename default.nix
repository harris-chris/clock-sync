{
  pkgs ? import <nixpkgs> {},
}:

let
  rustPackages = with pkgs; [
    cargo
    rustc
  ];

  cargoTestWrapped = pkgs.writeShellScriptBin "ctest" ''
    ${pkgs.cargo}/bin/cargo test $@ --color=always 2>&1 | less -r
  '';

  cargoRunWrapped = pkgs.writeShellScriptBin "crun" ''
    ${pkgs.cargo}/bin/cargo run $@ --color=always 2>&1
  '';

in pkgs.stdenv.mkDerivation {
  name = "clock-sync-rust-0.1";

  buildInputs = with pkgs; [
    cargoTestWrapped
    cargoRunWrapped
    rustPackages
  ];
}
