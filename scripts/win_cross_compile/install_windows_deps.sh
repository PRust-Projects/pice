#!/usr/bin/env bash

echo "Installing dependencies..."
sudo dnf install mingw64-gcc mingw64-pango mingw64-poppler mingw64-gtk3 mingw64-winpthreads

echo "Ready to attempt cross-compiling..."
