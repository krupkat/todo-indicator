with import <nixpkgs> { };

mkShell {
  nativeBuildInputs = [
    (python3.withPackages (pkgs: with pkgs; [
      requests
      pygobject3
      pyyaml
    ]))
    libappindicator-gtk3
    gobject-introspection
  ];
}
