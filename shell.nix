{}:

let
    # nix-prefetch-url <url> --unpack
    pkgs = import (builtins.fetchTarball {
        name = "nixos-24.11-2024-12-06";
        url = "https://github.com/nixos/nixpkgs/archive/e2605d0744c2417b09f8bf850dfca42fcf537d34.tar.gz";
        sha256 = "1fsfkdjlqknzxi9jc38a0k0103rlxnjj59xg1s9a5bqb3scaxh9m";
    }) {};
    rustPkgs = pkgs.rust.packages.stable;
    rust-analyzer = pkgs.rust-analyzer.override {
        rustPlatform = rustPkgs.rustPlatform;
    };
    vscode = pkgs.vscode-with-extensions.override {
        #vscode = pkgs.vscodium;
        vscodeExtensions = with pkgs.vscode-extensions; [
            # The "outer" rust-analyzer here is the vscode package, and the "inner" is the actual rust-analyzer program
            (rust-lang.rust-analyzer.override {
                inherit rust-analyzer;
            })
        ];
    };
in
    pkgs.mkShell {
        name = "shell";
        nativeBuildInputs = with rustPkgs;[
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            vscode
        ];
    }
