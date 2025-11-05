{ pkgs ? import <nixpkgs> {} }:

let lib = pkgs.lib; in

pkgs.mkShell 
{
	buildInputs = 
	[
		# Project overall dependencies
		pkgs.pkg-config
    		pkgs.openssl
    		pkgs.fontconfig
    		pkgs.sdl3
    		pkgs.sdl3-ttf
    		pkgs.cmake
    		pkgs.gcc
    		pkgs.rustc
    		pkgs.cargo
		pkgs.geckodriver
		pkgs.firefox
		pkgs.nerd-fonts.jetbrains-mono

    		# SDL3 build dependencies
    		pkgs.alsa-lib
    		pkgs.libx11
    		pkgs.wayland
    		pkgs.wayland-protocols
    		pkgs.libpulseaudio
    		pkgs.libdrm
    		pkgs.libusb1
    		pkgs.libunwind
    		pkgs.libxkbcommon
    		pkgs.dbus
    		pkgs.jack2
    		pkgs.pipewire
    		pkgs.egl-wayland
    		pkgs.mesa
    		pkgs.libGL
    		pkgs.libGLU
    		pkgs.libffi
    		pkgs.udev
    		pkgs.libdecor

    		# X11 dev libs
    		pkgs.xorg.libXcursor
    		pkgs.xorg.libXrandr
    		pkgs.xorg.libXi
    		pkgs.xorg.libXinerama
    		pkgs.xorg.libXext
    		pkgs.xorg.libXfixes
    		pkgs.xorg.libXScrnSaver
    		pkgs.xorg.libXrender
    		pkgs.xorg.libXdmcp
    		pkgs.xorg.libXt
    		pkgs.xorg.xorgproto
    		pkgs.xorg.libxcb

		# nvim dependencies
		pkgs.rust-analyzer

  	];



  	OPENSSL_DIR = pkgs.openssl.dev;
  	OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  	OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
	PKG_CONFIG_PATH = pkgs.lib.concatStringsSep ":" 
  	[
  		"${pkgs.openssl.dev}/lib/pkgconfig"
  		"${pkgs.libffi.dev}/lib/pkgconfig"
  	];



	shellHook = 
	''
    		export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath 
		[
      			pkgs.libGL
      			pkgs.mesa
      			pkgs.xorg.libX11
      			pkgs.xorg.libXext
      			pkgs.xorg.libXrandr
      			pkgs.xorg.libXi
      			pkgs.xorg.libXinerama
      			pkgs.xorg.libXcursor
      			pkgs.xorg.libXScrnSaver
      			pkgs.xorg.libXfixes
      			pkgs.xorg.libxcb
      			pkgs.wayland
      			pkgs.xorg.libXrender
      			pkgs.xorg.libXt
      			pkgs.xorg.libXdmcp
      			pkgs.xorg.xorgproto
    		]}:$LD_LIBRARY_PATH
	'';
}
