{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
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
            pkgs.tree-sitter
            pkgs.nodejs
            pkgs.typescript
            pkgs.typescript-language-server
            pkgs.rustc
            pkgs.cargo
            pkgs.rust-analyzer
          ];

          shellHook = ''
            source <(tree-sitter complete --shell bash)
          '';
        };

        formatter = pkgs.alejandra;
      };
    };
}
