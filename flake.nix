{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        system,
        pkgs,
        self',
        ...
      }: {
        devShells.default = pkgs.mkShell {
          packages = [
            self'.formatter
            pkgs.nil
            pkgs.tree-sitter
            pkgs.nodejs
            pkgs.typescript
            pkgs.typescript-language-server
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.rustup
            pkgs.cargo-expand
          ];

          shellHook = ''
            source <(tree-sitter complete --shell bash)
          '';
        };

        formatter = pkgs.alejandra;
      };
    };
}
