{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell {
  buildInputs = [
    cargo
    rustc
    gcc
    clang
    llvmPackages.libclang
    pkgconfig
  ];
  shellHook = ''
      export LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib";
    '';
}
