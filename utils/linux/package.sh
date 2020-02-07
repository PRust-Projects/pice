#!/usr/bin/env bash

VERSION="$1"
if [ -z "$VERSION" ]; then
    read -p 'What is the version: ' VERSION
fi

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
mkdir -p /tmp/pice/pice
cp $SCRIPT_DIR/package/{install.sh,uninstall.sh,pice.desktop} /tmp/pice
cp $SCRIPT_DIR/../../target/release/pice /tmp/pice/pice
cp -r $SCRIPT_DIR/package/{libsciter-gtk.so,pice.png,wordlists} /tmp/pice/pice
tar -czf "$(pwd)/pice-$VERSION-x86_64-linux.tar.gz" -C /tmp pice
rm -rf /tmp/pice
