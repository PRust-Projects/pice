#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
mkdir -p /tmp/pice/pice
cp $SCRIPT_DIR/package/{install.sh,uninstall.sh,pice.desktop} /tmp/pice
cp $SCRIPT_DIR/../../target/release/pice /tmp/pice/pice
cp -r $SCRIPT_DIR/package/{libsciter-gtk.so,pice.png,wordlists} /tmp/pice/pice
