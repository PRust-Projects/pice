#!/usr/bin/env bash

CURRENT_DIR="$(pwd)"
MINGW_PREFIX=/usr/x86_64-w64-mingw32/sys-root/mingw
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cd /tmp
rm -rf pice
mkdir pice
cp "$SCRIPT_DIR/../../target/x86_64-pc-windows-gnu/release/pice.exe" pice
cp $MINGW_PREFIX/bin/*.dll pice
cp -r "$SCRIPT_DIR/../../assets" pice
cp -r "$SCRIPT_DIR/../../wordlists" pice
zip -r pice.zip pice
cp pice.zip "$CURRENT_DIR"
rm -rf pice.zip pice
