{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    oldpkgs.url = "github:NixOS/nixpkgs/34a626458d686f1b58139620a8b2793e9e123bba";
  };

  outputs = {
    self,
    nixpkgs,
    oldpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pkgs-old = import oldpkgs {
          inherit system;
        };
      in {
        devShells.default = with pkgs;
          mkShell rec {
            nativeBuildInputs = [
              rust-bin.stable.latest.default
              pkgs-old.rust-analyzer
            ];
            buildInputs = [
              pkg-config
              libGL
              wayland
              libxkbcommon
            ];
            LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
            shellHook = ''
            '';
          };
      }
    );
}
