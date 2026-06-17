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

      flake.nixosModules.default = {
        config,
        lib,
        pkgs,
        ...
      }:
        with lib; let
          cfg = config.storm.services.backend;
        in {
          options.storm.services.backend = {
            enable = mkEnableOption "Storm backend service";

            package = mkOption {
              type = types.package;
              default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
              description = "The storm-backend package to use";
            };

            port = mkOption {
              type = types.port;
              default = 8080;
              description = "Port to listen on";
            };

            bindAddress = mkOption {
              type = types.str;
              default = "127.0.0.1";
              description = "Address to bind the server to";
            };

            secretsFile = mkOption {
              type = types.path;
              description = "Path to file containing secrets (STRIPE_SECRET, ANALYTICS_WEBSITE_ID, ANALYTICS_API_URL)";
            };

            user = mkOption {
              type = types.str;
              default = "storm";
              description = "User to run the service as";
            };

            group = mkOption {
              type = types.str;
              default = "storm";
              description = "Group to run the service as";
            };
          };

          config = mkIf cfg.enable {
            users.users.${cfg.user} = mkIf (cfg.user == "storm") {
              group = cfg.group;
              isSystemUser = true;
            };

            users.groups.${cfg.group} = mkIf (cfg.group == "storm") {};

            systemd.services.storm-backend = {
              description = "Storm Backend Service";
              after = ["network.target"];
              wantedBy = ["multi-user.target"];

              serviceConfig = {
                Type = "simple";
                ExecStart = "${cfg.package}/bin/storm-backend --port ${toString cfg.port} --bind-address ${cfg.bindAddress}";
                EnvironmentFile = cfg.secretsFile;
                Restart = "on-failure";
                RestartSec = 5;
                User = cfg.user;
                Group = cfg.group;
                PrivateTmp = true;
                NoNewPrivileges = true;
              };
            };
          };
        };

      perSystem = {
        self',
        pkgs,
        ...
      }: {
        rust-project = {
          src = self;
        };

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
