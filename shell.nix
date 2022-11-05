{ pkgs ? import <nixpkgs> { }
, unstable ? import <unstable> { }
}:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.creduce
    pkgs.cargo-flamegraph
    pkgs.halfempty
    pkgs.rustup
    unstable.python310
    unstable.python310Packages.mypy
  ];
}
