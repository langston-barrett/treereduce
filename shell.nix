{ pkgs ? import <nixpkgs> { }
, unstable ? import <unstable> { }
}:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.rustup
    unstable.python310
    unstable.python310Packages.mypy
  ];
}
