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

function copy_runtime() {
  for ext in rlib a dll so
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

function copy_toolchain() {
  echo "  COPY     letlangc.exe"
  cp ${CACHEDIR}/cli/letlangc.exe ${BINDIR}/letlangc.exe

  echo "  COPY     rustc.exe"
  rustc=$(which rustc)
  cp ${rustc} ${BINDIR}/rustc.exe
}

rm -rf $DISTDIR
mkdir -p $DISTDIR $BINDIR $LIBDIR $CACHEDIR

build_runtime
build_compiler

copy_runtime
copy_toolchain
