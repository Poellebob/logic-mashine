{ pkgs, lib, ... }:

let
  muslPkgs = pkgs.pkgsCross.musl64;
  mingwPkgs = pkgs.pkgsCross.mingwW64;

  muslCc = muslPkgs.stdenv.cc;
  mingwCc = mingwPkgs.stdenv.cc;

  muslLinker = "${muslCc}/bin/x86_64-unknown-linux-musl-cc";
  mingwLinker = "${mingwCc}/bin/x86_64-w64-mingw32-cc";

  # windows-gnu's std wants a *static* libpthread.a; the default mingw
  # pthreads package only builds the shared/import-lib variant.
  mingwPthreadsStatic = mingwPkgs.windows.pthreads.overrideAttrs (old: {
    configureFlags = (old.configureFlags or [ ]) ++ [ "--enable-static" ];
  });
in
{
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "llvm-tools-preview"
    ];

    targets = [
      "x86_64-unknown-linux-gnu"
      "x86_64-unknown-linux-musl"
      "x86_64-pc-windows-gnu"
    ];
  };

  packages = with pkgs; [
    just
    pkg-config
    cmake

    cargo-edit
    cargo-watch
    cargo-audit
    cargo-outdated
    cargo-nextest
    cargo-llvm-cov

    llvmPackages.clang
    llvmPackages.lld
    llvmPackages.bintools
    libclang
    glibc.dev

    # raylib (bundled GLFW X11) native deps
    libGL
    libx11
    libxcursor
    libxrandr
    libxinerama
    libxi
    libxext
    libxxf86vm
    libxfixes
    alsa-lib

    # cross toolchains (drop if unused)
    muslCc
    mingwCc
  ];

  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";

  # NixOS: expose mesa GL drivers so GLFW/X11 can load GLX at runtime
  env.LD_LIBRARY_PATH = "${pkgs.libglvnd}/lib:/run/opengl-driver/lib";
  env.__GLX_VENDOR_LIBRARY_NAME = "mesa";
  env.__GLX_VENDOR_LIBRARY_DIRS = "/run/opengl-driver/lib";

  # Native compilers for the default (host) target.
  env.CC = "${pkgs.stdenv.cc}/bin/cc";
  env.CXX = "${pkgs.stdenv.cc}/bin/c++";

  env.CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = "-C link-arg=-fuse-ld=lld";

  env.CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = muslLinker;
  env.CC_x86_64_unknown_linux_musl = muslLinker;

  env.CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = mingwLinker;
  env.CC_x86_64_pc_windows_gnu = mingwLinker;
  env.CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${mingwPthreadsStatic}/lib";

  git-hooks.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };
}
