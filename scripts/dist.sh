#!/bin/bash

set -ue -o pipefail

ROOTDIR="${PWD}"
CACHEDIR="${ROOTDIR}/cache"
DISTDIR="${ROOTDIR}/dist"
BINDIR="${DISTDIR}/bin"
LIBDIR="${DISTDIR}/lib"

function build_runtime() {
  echo "  BUILD    llruntime.rlib"
  cargo build \
    -Z unstable-options \
    -p letlang-runtime -q \
    --out-dir ${CACHEDIR}/runtime \
    --target-dir ${CACHEDIR}/runtime-target
}

function build_compiler() {
  echo "  BUILD    letlangc"
  cargo build \
    -Z unstable-options \
    -p letlang-cli -q \
    --out-dir ${CACHEDIR}/cli \
    --target-dir ${CACHEDIR}/cli-target
}

function copy_runtime_linux() {
  for ext in rlib a so
  do
    deps=$(find ${CACHEDIR}/runtime-target/debug/deps -name "*.${ext}")

    for dep in $deps
    do
      depdir=$(dirname $dep)
      depbase=$(basename $dep)
      deppat=${depbase%.${ext}}
      deppat=$(echo $deppat | cut -d- -f1)

      dest=$(basename $dep)
      dest=${dest%.${ext}}
      dest=$(echo $dest | cut -d- -f1)
      dest="${dest}.${ext}"

      dep=$(find $depdir -type f -name "${deppat}-*.${ext}" | head -n1)
      if [ ! -z "$dep" ]
      then
        echo "  COPY     ${dest}"
        cp $dep ${LIBDIR}/${dest}
      fi
    done
  done
}

function copy_runtime_windows() {
  for ext in rlib a dll
  do
    deps=$(find ${CACHEDIR}/runtime-target/debug/deps -name "*.${ext}")

    for dep in $deps
    do
      depdir=$(dirname $dep)
      depbase=$(basename $dep)
      deppat=${depbase%.${ext}}
      deppat=$(echo $deppat | cut -d- -f1)

      dest=$(basename $dep)
      dest=${dest%.${ext}}
      dest=$(echo $dest | cut -d- -f1)
      dest="${dest}.${ext}"

      dep=$(find $depdir -type f -name "${deppat}-*.${ext}" | head -n1)
      if [ ! -z "$dep" ]
      then
        echo "  COPY     ${dest}"
        cp $dep ${LIBDIR}/${dest}
      fi
    done
  done

  echo "  COPY    libwindows.0.48.5.a"
  find $HOME/.cargo/registry -name "libwindows.0.48.5.a" -exec cp {} ${LIBDIR}/ \;
}

function copy_toolchain_linux() {
  echo "  COPY     letlangc"
  cp ${CACHEDIR}/cli/letlangc ${BINDIR}/letlangc

  echo "  COPY     rustc"
  rustc=$(which rustc)
  cp ${rustc} ${BINDIR}/rustc

  echo "  COPY     rustlib"
  RUSTUP_HOME=$(rustup show home)
  RUSTUP_TOOLCHAIN=$(rustup show active-toolchain | cut -d '' -f1)
  cp -r ${RUSTUP_HOME}/toolchains/${RUSTUP_TOOLCHAIN}/lib/* ${LIBDIR}/*
}

function copy_toolchain_windows() {
  echo "  COPY     letlangc"
  cp ${CACHEDIR}/cli/letlangc.exe ${BINDIR}/letlangc.exe

  echo "  COPY     rustc"
  rustc=$(which rustc)
  cp ${rustc} ${BINDIR}/rustc.exe

  echo "  COPY     rustlib"
  RUSTUP_HOME=$(rustup show home)
  RUSTUP_TOOLCHAIN=$(rustup show active-toolchain | cut -d '' -f1)
  cp -r ${RUSTUP_HOME}/toolchains/${RUSTUP_TOOLCHAIN}/lib/* ${LIBDIR}/*
}

rm -rf $DISTDIR
mkdir -p $DISTDIR $BINDIR $LIBDIR $CACHEDIR

build_runtime
build_compiler

case "$OSTYPE" in
  linux*)
    copy_runtime_linux
    copy_toolchain_linux
    ;;
  msys*)
    copy_runtime_windows
    copy_toolchain_windows
    ;;

  *)
    echo "Unsupported OS: $OSTYPE"
    exit 1
    ;;
esac
