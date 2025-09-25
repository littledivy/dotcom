{
  description = "littledivy.com dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        packages.html-shim = pkgs.buildTypstPackage {
          pname = "html-shim";
          version = "0.1.0";

          src = ./typst/lib/html-shim/0.1.0;
        };
        typst = pkgs.typst.withPackages (p: with p; [ packages.html-shim ]);
      in
      {
        devShells.default = pkgs.mkShell {

          buildInputs =
            with pkgs;
            [
              rustup
              typst
              typstyle
              deno
              nodejs_22
              pnpm
            ]
            ++ lib.optionals stdenv.isDarwin [
              libiconv
            ];
        };
      }
    );
}
