{ pkgs ? import <nixpkgs> { }
, unstable ? import <unstable> { }
}:

pkgs.mkShell {
  LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib/:/run/opengl-driver/lib/";
  nativeBuildInputs = [
    pkgs.creduce
    pkgs.cargo-flamegraph
    pkgs.halfempty
    pkgs.lit
    pkgs.rust-analyzer
    pkgs.rustup

    # Running perses/picireny:
    pkgs.jre
    
    pkgs.python39Packages.python-lsp-server
    pkgs.python39Packages.virtualenv
  ];
}
