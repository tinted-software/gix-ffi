{
  description = "The purely functional package manager";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs =
    {
      # self,
      nixpkgs,
      ...
    }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          cargo-deny
          cargo-nextest
          rust-bindgen
          rustPackages.clippy
          rust-analyzer
          pkgconf
          flex
          bison
        ];

        buildInputs = with pkgs; [
          boost
          libressl
        ];

        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      };
    };
}
