{ pkgs,
  rust_pkg,
  crane,
  ...
}: let
  workspace_toml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  package_name = "raesan";
  package_version = workspace_toml.workspace.package.version;
  crane_lib = (crane.mkLib pkgs).overrideToolchain rust_pkg;
  clean_src = crane_lib.cleanCargoSource ../.;
  full_src = pkgs.lib.cleanSourceWith {
    src = ../.;
    filter = path: type: let
      name = builtins.baseNameOf path;
    in
      !(
        name
        == "node_modules"
        || name == "dist"
        || name == ".git"
        || name == "target"
        || name == ".direnv"
      );
  };
  common_args = {
    pname = package_name;
    src = clean_src;
    version = package_version;
    strictDeps = true;
    doCheck = false;
    # the two arguments below are used to disable the "cargo check" command from running
    cargoCheckCommand = "";
    cargoExtraArgs = "";
    nativeBuildInputs = [
      pkgs.pkg-config
      pkgs.lld
      pkgs.clang
    ];
    buildInputs = [
      pkgs.sqlite
      pkgs.libGL
      pkgs.xorg.libX11
      pkgs.libxkbcommon
      pkgs.xorg.libXrandr
      pkgs.xorg.libXinerama
      pkgs.xorg.libXcursor
      pkgs.xorg.libXi
      pkgs.cargo-tauri
      pkgs.glib
      pkgs.gdk-pixbuf
      pkgs.pango
      pkgs.cairo
      pkgs.libsoup_3
      pkgs.webkitgtk_4_1
      pkgs.gtk3
      pkgs.xdg-utils
    ];
  };
  cargo_artifacts = crane_lib.buildDepsOnly common_args;
in {
  web = crane_lib.buildPackage (common_args
    // rec {
      pname = "${package_name}_web";
      cargoExtraArgs = "-p ${pname}";
      cargoArtifacts = cargo_artifacts;
    });
  web_scraper = crane_lib.buildPackage (common_args
    // rec {
      pname = "${package_name}_web_scraper";
      cargoExtraArgs = "--bin ${pname}";
      cargoArtifacts = cargo_artifacts;
    });
	frontend = pkgs.stdenv.mkDerivation rec {
		pname = "${package_name}_frontend";
		version = package_version;
		src = ../frontend;
		nativeBuildInputs = [
			pkgs.yarn-berry_4.yarnBerryConfigHook
			pkgs.yarn-berry_4
		];
	    missingHashes = ../frontend/missing-hashes.json;
		offlineCache = pkgs.yarn-berry_4.fetchYarnBerryDeps {
			inherit src missingHashes;
			hash = "sha256-EAIdKufr2d0ISOKhtQL+Vv7JJJpvaXEJswWVWj4ltzQ=";
		};
		PUBLIC_APP_PLATFORM = "web";
		PUBLIC_API_URL = "http://localhost:8080";
		buildPhase = ''
			yarn run build
		'';
		installPhase = ''
			mkdir -p $out
			cp -R build/ $out/
		'';
	};
}
