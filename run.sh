#!/bin/bash

# Build the SDRE Hub
# By default we build SDREHub as a web app (future will have a standalone app) and build as a debug version

# gather the command line options. Valid are `--web` and `--standalone` for the type of app to build
# and `--release` and `--debug` for the type of build to create

# default values
app_type="web"
build_type="debug"
clean=false
extra_args=()

# parse the command line options

while [ $# -gt 0 ]; do
    case "$1" in
        --web)
        app_type="web"
        ;;
        --standalone)
        app_type="standalone"
        ;;
        --release)
        build_type="release"
        ;;
        --debug)
        build_type="debug"
        ;;
        --clean)
            clean=true
            ;;
        *)
            extra_args+=("$1")
        ;;
    esac
    shift
done

echo "Building SDRE Hub"

# clean the build directory
if [ "$clean" = true ]; then
echo "Cleaning the build directory"
cargo clean
fi

# output the build options

echo "App type: $app_type"
echo "Build type: $build_type"

# build the app

if [ "$app_type" == "web" ]; then
    echo "Building SDRE Hub as a web app"
    if [ "$build_type" == "debug" ]; then
        echo "Building debug version"
        # build the debug version
        cargo run --bin sdre-hub -- "${extra_args[@]}"
        else
        echo "Building release version"
        # build the release version
        cargo run --release --bin sdre-hub -- "${extra_args[@]}"
    fi
    else
    echo "Building SDRE Hub as a standalone app"
    echo "Not yet implemented. Exiting"
    if [ "$build_type" == "debug" ]; then
        echo "Building debug version"
        # build the debug version
        cargo tauri serve -d
    else
        echo "Building release version"
        # build the release version
        cargo tauri serve
    fi
fi
