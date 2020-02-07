#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
$SCRIPT_DIR/packfolder $SCRIPT_DIR/../../src/resources $SCRIPT_DIR/../../src/resources.rc -binary
if [ ! -d "$SCRIPT_DIR/osxcross/target/bin" ]; then
    $SCRIPT_DIR/osxcross_setup.sh
fi
PATH="$SCRIPT_DIR/osxcross/target/bin:$PATH" cargo build --target x86_64-apple-darwin --release
