{
  pkgs ? import <nixpkgs> {},
}:

let
  rustPackages = with pkgs; [
    cargo
    rustc
  ];

  ctest = pkgs.writeShellScriptBin "ctest" ''
    ${pkgs.cargo}/bin/cargo test --color=always $@ 2>&1 | less -r
  '';

  naive = pkgs.writeShellScriptBin "naive" ''
    ${pkgs.cargo}/bin/cargo run --bin naive --color=always -- $@ 2>&1
  '';

  remainder = pkgs.writeShellScriptBin "remainder" ''
    ${pkgs.cargo}/bin/cargo run --bin remainder --color=always -- $@ 2>&1
  '';

in pkgs.stdenv.mkDerivation {
  name = "clock-sync-rust-0.1";

  buildInputs = with pkgs; [
    ctest
    naive
    remainder
    rustPackages
  ];
}
