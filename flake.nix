{
  description = "(Lib)Canon dev environment";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.zst";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];

    perSystem = { pkgs, self', ... }: {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "canon";
        version = "1.0.0";
        src = ./.;

        buildInputs = [ pkgs.openssl ];
        nativeBuildInputs = [ pkgs.pkg-config ];
        cargoLock.lockFile = ./Cargo.lock;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = [ self'.packages.default ];
        packages = with pkgs; [
          cargo rustc rustfmt rust-analyzer
        ];
      };
    };
  };
}

