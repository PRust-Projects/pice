#!/usr/bin/env bash

VERSION="$1"
if [ -z "$VERSION" ]; then
    read -p 'What is the version: ' VERSION
fi

CURRENT_DIR="$(pwd)"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
SCRIPT_DIR="$(cygpath -m $SCRIPT_DIR)"
mkdir /tmp/pice
cp -r $SCRIPT_DIR/package/{sciter.dll,wordlists} /tmp/pice
cp $SCRIPT_DIR/../../target/release/pice.exe /tmp/pice
cd /tmp && zip -r "$CURRENT_DIR/pice-$VERSION-x86_64-windows.zip" pice
rm -rf /tmp/pice
