#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
SCRIPT_DIR="$(cygpath -m $SCRIPT_DIR)"
$SCRIPT_DIR/build/packfolder.exe $SCRIPT_DIR/../../src/resources $SCRIPT_DIR/../../src/resources.rc -binary
cargo build --release
if [ ! -f "$SCRIPT_DIR/../../target/release/sciter.dll" ]; then
	cp $SCRIPT_DIR/package/sciter.dll $SCRIPT_DIR/../../target/release
	cp -r $SCRIPT_DIR/package/wordlists $SCRIPT_DIR/../../target/release
fi
if [ -z "$1" ]; then
	$SCRIPT_DIR/../../target/release/pice.exe
fi
