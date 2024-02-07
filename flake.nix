{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs.outPath {
          config = { allowUnfree = true; };
          inherit system;
        };
      in
      {
        defaultPackage = pkgs.mkShell {
          buildInputs = with pkgs; [
            llvmPackages.clangNoLibcxx
            llvmPackages.lldb
            rustc
            cargo
            clippy
            rustfmt
            rust-analyzer
          ];
        };
      }
    );
}
