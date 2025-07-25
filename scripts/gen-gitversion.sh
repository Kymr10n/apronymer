#!/bin/bash
# Generate gitversion.txt for the frontend before build
GIT_VERSION=$(git describe --always --dirty)
echo "$GIT_VERSION" > "$(dirname "$0")/../frontend/public/gitversion.txt"
