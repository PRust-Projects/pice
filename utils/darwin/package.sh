#!/usr/bin/env bash

VERSION="$1"
if [ -z "$VERSION" ]; then
    read -p 'What is the version: ' VERSION
fi

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
mkdir -p /tmp/pice.app/Contents/{MacOS,Resources}
cp $SCRIPT_DIR/package/Info.plist /tmp/pice.app/Contents
cp $SCRIPT_DIR/../../target/x86_64-apple-darwin/release/pice /tmp/pice.app/Contents/MacOS
cp -r $SCRIPT_DIR/package/{sciter-osx-64.dylib,wordlists} /tmp/pice.app/Contents/MacOS
cp $SCRIPT_DIR/package/pice.icns /tmp/pice.app/Contents/Resources
tar -czf "$(pwd)/pice-$VERSION-x86_64-darwin.tar.gz" -C /tmp pice.app
rm -rf /tmp/pice.app
