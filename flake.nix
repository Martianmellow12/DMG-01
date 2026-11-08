{
  description = "DMG-01 Project Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "llvm-tools" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rustToolchain 
            pkgs.cargo-binutils # For cargo-size, cargo-objcopy, etc.
            pkgs.pkg-config
          ];

          buildInputs = with pkgs; [
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            libxkbcommon
          ];

          shellHook = ''
            # export CARGO_TARGET_THUMBV6M_NONE_EABI_LINKER="arm-none-eabi-ld"
            echo "Compiler version: $(rustc --version)"
            zsh
            exit
          '';
        };
      });
}

