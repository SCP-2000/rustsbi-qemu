{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlay ]; };
        in
        {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs;[
              ((rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
                extensions = [
                  "llvm-tools-preview"
                ];
                targets = [ "riscv64imac-unknown-none-elf" ];
              })
              cargo-binutils
              qemu
            ];
          };
        }
      );
}
