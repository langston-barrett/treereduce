{ pkgs ? import <nixpkgs> { }
, unstable ? import <unstable> { }
}:

pkgs.mkShell {
  LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib/:/run/opengl-driver/lib/";
  nativeBuildInputs = [
    pkgs.creduce
    pkgs.cargo-flamegraph
    pkgs.halfempty
    pkgs.rust-analyzer
    pkgs.rustup
    
    pkgs.python3Packages.python-lsp-server
    pkgs.python3Packages.virtualenv
  ];
}
