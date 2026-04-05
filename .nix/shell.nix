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
      pkgs.jq
      pkgs.lld
      pkgs.clang
	  pkgs.sqlitebrowser
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
    shellHook = ''
      export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
        pkgs.glib
        pkgs.gtk3
        pkgs.gdk-pixbuf
        pkgs.cairo
        pkgs.libsoup_3
        pkgs.webkitgtk_4_1
      ]}:$LD_LIBRARY_PATH"
      zsh
    '';
  };
}
