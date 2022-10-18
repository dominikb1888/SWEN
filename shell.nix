{ pkgs ? import <nixpkgs> {} }:

let
  pythonEnv = with pkgs.python310Packages; [
    ipython
    jupyter
    pandas
    numpy
    rich
  ];

in pkgs.mkShell {
  buildInputs = with pkgs; [
    pythonEnv

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
