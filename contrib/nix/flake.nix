{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, ... }@inputs:

    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = inputs.nixpkgs.lib.genAttrs systems;

      mkPkgs = system:
        import inputs.nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        };

    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
              targets."thumbv8m.main-none-eabihf".stable.rust-std
              llvm-tools
            ]
          );

        # 自定义工具打包
        chiptool = prev.callPackage ./chiptool.nix { };
        form = prev.callPackage ./form.nix { };
      };

      packages = forAllSystems (system: {
        chiptool = (mkPkgs system).chiptool;
        form = (mkPkgs system).form;
      });

      devShells = forAllSystems (system:
        let
          pkgs = mkPkgs system;
        in
        {
          default = pkgs.mkShellNoCC {
            packages = (with pkgs; [
              rustToolchain
              openssl
              pkg-config
              cargo-deny
              cargo-edit
              cargo-watch
              rust-analyzer
              cargo-binutils
              # SiFli 特定工具
              chiptool
              form
            ]);

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        }
      );
    };
}
