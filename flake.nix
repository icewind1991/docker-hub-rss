{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.docker-hub-rss = naersk-lib.buildPackage {
            pname = "docker-hub-rss";
            root = ./.;
          };
          defaultPackage = packages.docker-hub-rss;
          defaultApp = packages.docker-hub-rss;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo bacon cargo-edit cargo-outdated ];
          };
        }
    );
}
