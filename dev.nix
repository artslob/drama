{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/4d2b37a84fad1091b9de401eb450aae66f1a741e.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.git
    pkgs.pre-commit
    pkgs.nixfmt
    pkgs.which
    pkgs.htop
    pkgs.jdk
    pkgs.docker-compose
    pkgs.nodejs # required for spotless:prettier
  ];
}
