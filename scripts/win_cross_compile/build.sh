#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

export PKG_CONFIG_ALLOW_CROSS=1
export MINGW_PREFIX=/usr/x86_64-w64-mingw32/sys-root/mingw
export PKG_CONFIG_PATH=$MINGW_PREFIX/lib/pkgconfig

cd  "$SCRIPT_DIR/../../"
cargo build --target=x86_64-pc-windows-gnu --release
