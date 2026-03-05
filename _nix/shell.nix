{
  pkgs,
  rust_pkg,
  ...
}: {
  default = pkgs.mkShell {
    nativeBuildInputs = [
      pkgs.alejandra
      rust_pkg
      pkgs.bun
      pkgs.pkg-config
      pkgs.lld
      pkgs.clang
      pkgs.jq
      pkgs.sqlite
    ];
    buildInputs = [];
    shellHook = "zsh";
  };
}
