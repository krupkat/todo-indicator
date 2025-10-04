{
  lib,
  rustPlatform,
  libcosmicAppHook,
  pkg-config,
  openssl
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cosmic-applet-todos";
  version = "0.0.1";

  src = ./.;

  cargoHash = "sha256-mjPxnyj7ayks0kiHmLXnYNNQHBlbcUx3vqQpEClTTZI=";

  nativeBuildInputs = [
    pkg-config
    libcosmicAppHook
  ];

  buildInputs = [
    openssl.dev
  ];

  postInstall = ''
    mkdir -p $out/share/applications
    cp res/com.gitlab.todo-indicator.desktop $out/share/applications/
  '';
})