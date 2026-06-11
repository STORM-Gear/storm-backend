{
  description = "STORM server fonctions";

  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

      perSystem = {
        self',
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell rec {
          name = "storm-backend";

          inputsFrom = [
            self'.devShells.rust
          ];

          packages = with pkgs; [
            stripe-cli
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };

        packages.default = self'.packages.storm-backend;
      };
    };
}
