#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
$SCRIPT_DIR/packfolder $SCRIPT_DIR/../../src/resources $SCRIPT_DIR/../../src/resources.rc -binary
cargo build --release
if [ ! -f "$SCRIPT_DIR/../../target/release/libsciter-gtk.so" ]; then
	cp $SCRIPT_DIR/libsciter-gtk.so $SCRIPT_DIR/../../target/release
fi
$SCRIPT_DIR/../../target/release/pice
