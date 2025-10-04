{
  lib,
  stdenv,
  fetchFromGitHub,
  rustPlatform,
  libcosmicAppHook,
  just,
  pkg-config,
  util-linuxMinimal,
  dbus,
  glib,
  libinput,
  pulseaudio,
  pipewire,
  udev,
  xkeyboard_config,
  nix-update-script,
  nixosTests,
  openssl
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cosmic-applet-todos";
  version = "0.0.1";

  src = ./.;

  cargoHash = "sha256-mjPxnyj7ayks0kiHmLXnYNNQHBlbcUx3vqQpEClTTZI=";

  nativeBuildInputs = [
    # just
    pkg-config
    # util-linuxMinimal
    libcosmicAppHook
    # rustPlatform.bindgenHook
  ];

  buildInputs = [
    openssl.dev
    # dbus
    # glib
    # libinput
    # pulseaudio
    # pipewire
    # udev
  ];

  postInstall = ''
    mkdir -p $out/share/applications
    cp res/com.gitlab.todo-indicator.desktop $out/share/applications/
  '';

  # dontUseJustBuild = true;
  # dontUseJustCheck = true;

  # justFlags = [
  #   "--set"
  #   "prefix"
  #   (placeholder "out")
  #   "--set"
  #   "target"
  #   "${stdenv.hostPlatform.rust.cargoShortTarget}/release"
  # ];

  # preFixup = ''
  #   libcosmicAppWrapperArgs+=(
  #     --set-default X11_BASE_RULES_XML ${xkeyboard_config}/share/X11/xkb/rules/base.xml
  #     --set-default X11_EXTRA_RULES_XML ${xkeyboard_config}/share/X11/xkb/rules/base.extras.xml
  #   )
  # '';

  # passthru = {
  #   tests = {
  #     inherit (nixosTests)
  #       cosmic
  #       cosmic-autologin
  #       cosmic-noxwayland
  #       cosmic-autologin-noxwayland
  #       ;
  #   };
  #   updateScript = nix-update-script {
  #     extraArgs = [
  #       "--version"
  #       "unstable"
  #       "--version-regex"
  #       "epoch-(.*)"
  #     ];
  #   };
  # };

  # meta = {
  #   homepage = "https://github.com/pop-os/cosmic-applets";
  #   description = "Applets for the COSMIC Desktop Environment";
  #   license = lib.licenses.gpl3Only;
  #   teams = [ lib.teams.cosmic ];
  #   platforms = lib.platforms.linux;
  # };
})