{
  gitignore,
  crane_lib,
  native_build_inputs,
  build_inputs,
  dockerTools,
  cacert,
  ...
}: let
  package_name = "raesan_web";
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
in rec {
  default = crane_lib.buildPackage (common_args
    // rec {
      pname = package_name;
      cargoExtraArgs = "-p ${pname}";
      cargoArtifacts = cargo_artifacts;
    });
  docker = dockerTools.buildLayeredImage {
    name = "raesan";
    tag = "latest";
    contents = [
      cacert
      default
    ];
    config = {
      Cmd = ["/bin/raesan_web"];
      ExposedPorts = {
        "8080/tcp" = {};
      };
      Env = [
        "SSL_CERT_FILE=${cacert}/etc/ssl/certs/ca-bundle.crt"
      ];
    };
  };
}
