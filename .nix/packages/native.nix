{
  gitignore,
  crane_lib,
  native_build_inputs,
  build_inputs,
  ...
}: let
  package_name = "raesan_native";
  src = gitignore.lib.gitignoreSource ../../.;
  workspace_toml = builtins.fromTOML (builtins.readFile (src + "/Cargo.toml"));
  package_version = workspace_toml.workspace.package.version;
  common_args = {
    pname = package_name;
    src = crane_lib.cleanCargoSource src;
    version = package_version;
    strictDeps = true;
    doCheck = false;
    # below arguments are set to prevent "cargo check" command from running
    cargoCheckCommand = "";
    cargoExtraArgs = "";
    nativeBuildInputs = native_build_inputs;
    buildInputs = build_inputs;
  };
  cargo_artifacts = crane_lib.buildDepsOnly common_args;
in
  crane_lib.buildPackage (common_args
    // rec {
      pname = package_name;
      cargoExtraArgs = "-p ${pname}";
      cargoArtifacts = cargo_artifacts;
    })
