# https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
{
  description = "a program to manage a flight logbook à la Jeppesen.";
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-24.11";
    };
  };
  outputs = {nixpkgs,...}:
    let system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; };
    in
      {
        packages."${system}".default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "zenjep";
          version = "0.1";
          src = pkgs.lib.cleanSource ./.;
          #cargoHash = pkgs.lib.fakeHash;
          cargoLock.lockFile = ./Cargo.lock;
        };
      };
}
