{pkgs ? import <nixpkgs> {}}:
with pkgs;
  rustPlatform.buildRustPackage {
    pname = "wearone-xmltv";
    version = "0.1.0";

    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    meta = with lib; {
      description = "An XMLTV grabber for WeAreOne.FM internet radio stations.";
      homepage = "https://codeberg.org/mart-w/weareone-xmltv";
    };
  }
