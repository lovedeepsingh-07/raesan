{
  stdenv,
  frontend_src,
  gitignore,
  yarn_berry,
  zip,
}: let
  src = gitignore.lib.gitignoreSource frontend_src;
  package_json = builtins.fromJSON (builtins.readFile "${src}/package.json");
  missing_hashes = "${src}/missing-hashes.json";
in
  stdenv.mkDerivation rec {
    pname = package_json.name;
    version = package_json.version;
    inherit src;
    nativeBuildInputs = [
      yarn_berry.yarnBerryConfigHook
      yarn_berry
      zip
    ];
    missingHashes = missing_hashes;
    offlineCache = yarn_berry.fetchYarnBerryDeps {
      inherit src;
      missingHashes = missing_hashes;
      hash = "sha256-7l2aZehmI4FmCnuMTDeNDQd7DhxxCTyUgilZ7U1U8zc=";
    };
    buildPhase = ''
      yarn run build
    '';
    installPhase = ''
         mkdir -p $out/dist $out/zip
         cp -r .svelte-kit node_modules .yarn $out/dist/
         cd $out/dist/
      zip -r $out/zip/${pname}-${version}.zip .
    '';
  }
