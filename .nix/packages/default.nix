{
  pkgs,
  crane_lib,
  gitignore,
  ...
}: let
  package_version = (builtins.fromTOML (builtins.readFile ../../Cargo.toml)).workspace.package.version;
  native_build_inputs = [
    pkgs.pkg-config
    pkgs.lld
    pkgs.clang
  ];
  build_inputs = [
    pkgs.sqlite
    pkgs.libGL
    pkgs.xorg.libX11
    pkgs.libxkbcommon
    pkgs.xorg.libXrandr
    pkgs.xorg.libXinerama
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.glib
    pkgs.gdk-pixbuf
    pkgs.pango
    pkgs.cairo
    pkgs.libsoup_3
    pkgs.webkitgtk_4_1
    pkgs.gtk3
    pkgs.xdg-utils
  ];
in rec {
  web_scraper = pkgs.callPackage ./web_scraper.nix {
    inherit gitignore crane_lib native_build_inputs build_inputs;
  };
  native = pkgs.callPackage ./native.nix {
    inherit gitignore crane_lib native_build_inputs build_inputs;
  };
  web = pkgs.callPackage ./web.nix {
    inherit gitignore crane_lib native_build_inputs build_inputs;
  };
  frontend = pkgs.callPackage ./frontend.nix {
    inherit gitignore;
    yarn_berry = pkgs.yarn-berry_4;
  };
  default = pkgs.stdenv.mkDerivation {
  	pname = "raesan";
    dontUnpack = true;
	version = package_version;
    nativeBuildInputs = [pkgs.zip web_scraper web frontend];
	installPhase = ''
      mkdir -p $out
      cp -r ${web_scraper}/bin/* $out/
      cp -r ${web}/bin/* $out/
	  cp -r ${frontend}/zip/* $out/
	'';
  };
}
