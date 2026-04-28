{
  gitignore,
  crane_lib,
  native_build_inputs,
  build_inputs,
  cargo-tauri,
  yarn_berry,
  callPackage,
  ...
}: let
  package_name = "raesan_native";
  src = ../../.;
  workspace_toml = builtins.fromTOML (builtins.readFile "${src}/Cargo.toml");
  package_version = workspace_toml.workspace.package.version;
  frontend_package = callPackage ./frontend.nix {
    inherit gitignore yarn_berry;
    frontend_src = src + /frontend;
  };

  common_args = {
    pname = package_name;
    src = gitignore.lib.gitignoreSource src;
    version = package_version;
    strictDeps = true;
    doCheck = false;
    # below arguments are set to prevent "cargo check" command from running
    cargoCheckCommand = "";
    cargoExtraArgs = "";
    nativeBuildInputs =
      [
        cargo-tauri
      ]
      ++ native_build_inputs;
    buildInputs = build_inputs;
  };
  cargo_artifacts = crane_lib.buildDepsOnly common_args;
in
  crane_lib.mkCargoDerivation (common_args
    // {
      pname = package_name;
      cargoArtifacts = cargo_artifacts;
      preBuild = ''
         	mkdir -p frontend/build
        cp -r ${frontend_package}/dist/* frontend/build/
      '';
      buildPhaseCargoCommand = "cargo tauri build --no-bundle";
      installPhase = ''
         	mkdir -p $out/bin
        cp -r target/release/${package_name} $out/bin/
      '';
    })
