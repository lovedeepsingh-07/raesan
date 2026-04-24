{
  stdenv,
  gitignore,
  yarn_berry,
}: let
  src = gitignore.lib.gitignoreSource ../../frontend;
  package_json = builtins.fromJSON (builtins.readFile (src + "/package.json"));
  missing_hashes = src + "/missing-hashes.json";
in
  stdenv.mkDerivation {
    pname = package_json.name;
    version = package_json.version;
    inherit src;
    nativeBuildInputs = [
      yarn_berry.yarnBerryConfigHook
      yarn_berry
    ];
    missingHashes = missing_hashes;
    offlineCache = yarn_berry.fetchYarnBerryDeps {
      inherit src;
      missingHashes = missing_hashes;
      hash = "sha256-m2BiDHTiT2YJSHdpnI3CNmcu/zktEyXYdvWO8wRxFGI=";
    };
    PUBLIC_APP_PLATFORM = "web";
    PUBLIC_API_URL = "http://localhost:8080";
    buildPhase = ''
      yarn run build
    '';
    installPhase = ''
      mkdir -p $out
      cp -R build/* $out/
    '';
  }
