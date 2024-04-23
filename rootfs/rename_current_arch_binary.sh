#!/usr/bin/env bash

set -x

ls -la /opt/

# determine which binary to keep
if /opt/sdre-hub.amd64 --version > /dev/null 2>&1; then
    mv -v /opt/sdre-hub.amd64 /opt/sdre-hub
elif /opt/sdre-hub.arm64 --version > /dev/null 2>&1; then
    mv -v /opt/sdre-hub.arm64 /opt/sdre-hub
elif /opt/sdre-hub.armv7 --version > /dev/null 2>&1; then
    mv -v /opt/sdre-hub.armv7 /opt/sdre-hub
else
    >&2 echo "ERROR: Unsupported architecture"
    exit 1
fi
