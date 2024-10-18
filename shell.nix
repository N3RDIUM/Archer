let pkgs = import <nixpkgs> { };
in pkgs.mkShell { buildInputs = [ pkgs.SDL2 pkgs.SDL2.dev ]; }
