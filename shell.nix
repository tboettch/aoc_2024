{}:

let
    # nix-prefetch-url <url> --unpack
    pkgs = import (builtins.fetchTarball {
        name = "nixos-24.05-2024-08-21";
        url = "https://github.com/nixos/nixpkgs/archive/759537f06e6999e141588ff1c9be7f3a5c060106.tar.gz";
        sha256 = "1an5d5a68ip1bk0l7375jwlrnmg1q9iaxjwymi11z76k4kqch0r9";
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
