#!/usr/bin/env sh

set -e

dx bundle                      \
    --desktop                  \
    --release                  \
    --package-types "rpm"      \
    --package-types "deb"      \
    --package-types "appimage" \
