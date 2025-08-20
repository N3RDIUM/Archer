{
  pkgs ? import <nixpkgs> {} 
}:

pkgs.mkShell {
  buildInputs = [
    pkgs.vulkan-tools           # For vulkaninfo, etc.
    pkgs.vulkan-loader
    pkgs.vulkan-validation-layers
    pkgs.libxkbcommon
    pkgs.wayland
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi
    pkgs.pkg-config
  ];

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  # Needed for dynamic libraries in some cases
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    vulkan-loader
    wayland
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
  ];
}
