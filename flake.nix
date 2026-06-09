{
  description = "VCS-aware safe mv/rm replacement with trash and audit";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
        
        buildInputs = with pkgs; [
          sqlite
          pkg-config
        ];
        
        nativeBuildInputs = with pkgs; [
          rustToolchain
          clippy
          rustfmt
          just
          cargo-watch
          cargo-tarpaulin
          cargo-audit
        ];
        
        smartfoPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "smartfo";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildInputs = buildInputs;
          nativeBuildInputs = [ rustToolchain ];
          doCheck = false; # Skip tests during build to avoid devbox dependency
          buildPhase = ''
            cargo build --release
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/smartfo $out/bin/smartfo
            
            # Add post-install message
            mkdir -p $out/share/doc/smartfo
            cat > $out/share/doc/smartfo/POST_INSTALL << 'EOF'
To complete installation, run:
  smartfo --install

This will install symlinks for mv/rm replacement and set up git hooks.
EOF
          '';
        };
      in
      {
        packages.default = smartfoPackage;
        
        apps.default = flake-utils.lib.mkApp {
          drv = smartfoPackage;
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ nativeBuildInputs;
          
          shellHook = ''
            export RUST_LOG="info"
            export RUST_BACKTRACE="1"
            export DATABASE_URL="sqlite:./smartfo.db"
            
            echo "🦀 Rust CLI environment ready for smartfo!"
            just bootstrap-internal
          '';
        };
      }
    );
}
