{lib, ...}: {
  get_filtered_src = src:
    lib.cleanSourceWith {
      inherit src;
      filter = path: type: let
        name = builtins.baseNameOf path;
      in (
        name == "node_modules" || name == ""
      );
    };
}
