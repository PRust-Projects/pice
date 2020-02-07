#!/usr/bin/env bash

VERSION="$1"
if [ -z "$VERSION" ]; then
    read -p 'What is the version: ' VERSION
fi

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
$SCRIPT_DIR/linux/build.sh --no-run
$SCRIPT_DIR/linux/package.sh "$VERSION"
$SCRIPT_DIR/darwin/build.sh
$SCRIPT_DIR/darwin/package.sh "$VERSION"
