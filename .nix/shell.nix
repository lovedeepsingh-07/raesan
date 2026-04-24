{
  pkgs,
  rust_pkg,
  ...
}: {
  default = pkgs.mkShell {
    packages = [
      pkgs.alejandra
      rust_pkg
      pkgs.yarn-berry_4
      pkgs.yarn-berry_4.yarn-berry-fetcher
      pkgs.act
      pkgs.jq
      pkgs.lld
      pkgs.clang
      pkgs.sqlitebrowser
      pkgs.cargo-tauri
      pkgs.live-server

      pkgs.pkg-config
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
