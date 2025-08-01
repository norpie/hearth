{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    rust-overlay.url = "github:oxalica/rust-overlay";
    # crane.url = "github:ipetkov/crane";
    # crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import inputs.systems;

      perSystem = {
        config,
        self',
        pkgs,
        lib,
        system,
        ...
      }: let
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
          targets = [
            # WebAssembly targets
            "wasm32-unknown-unknown"
            # Android targets
            "aarch64-linux-android"
            "armv7-linux-androideabi"
            "i686-linux-android"
            "x86_64-linux-android"
          ];
        };
        rustBuildInputs =
          [
            pkgs.openssl
            pkgs.libiconv
            pkgs.pkg-config
          ]
          ++ lib.optionals pkgs.stdenv.isLinux [
            pkgs.glib
            pkgs.gtk3
            pkgs.libsoup_3
            pkgs.webkitgtk_4_1
            pkgs.xdotool
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
            IOKit
            Carbon
            WebKit
            Security
            Cocoa
          ]);
        # This is useful when building crates as packages
        # Note that it does require a `Cargo.lock` which this repo does not have
        # craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
          };
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };

        devShells.default = pkgs.mkShell {
          name = "dioxus-dev";
          buildInputs = rustBuildInputs; # ++ [androidComposition.androidsdk];
          nativeBuildInputs = [
            rustToolchain
            pkgs.inotify-tools
            # pkgs.dioxus-cli
          ];
          shellHook = ''
            # For rust-analyzer 'hover' tooltips to work.
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";

            # Add cargo bin path for locally installed tools like dx
            export PATH="$HOME/.local/share/cargo/bin:$PATH";

            # Java stuff
            export JAVA_HOME="${pkgs.zulu17}";
            export _JAVA_AWT_WM_NONREPARENTING=1;

            # Android SDK
            export ANDROID_HOME="$HOME/.config/android";
            export ANDROID_SDK_ROOT="$ANDROID_HOME";
            # find ndk version
            export ANDROID_NDK_HOME=$(find "$ANDROID_HOME/ndk" -maxdepth 1 -mindepth 1 -type d);
          '';
          XDG_DATA_DIRS = let
            base = pkgs.lib.concatMapStringsSep ":" (x: "${x}/share") [
              pkgs.adwaita-icon-theme
              pkgs.shared-mime-info
            ];
            gsettings_schema = pkgs.lib.concatMapStringsSep ":" (x: "${x}/share/gsettings-schemas/${x.name}") [
              pkgs.glib
              pkgs.gsettings-desktop-schemas
              pkgs.gtk3
            ];
          in "${base}:${gsettings_schema}";
        };
      };
    };
}
