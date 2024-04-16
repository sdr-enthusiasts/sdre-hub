#!/bin/bash

# Build the SDRE Hub
# By default we build SDREHub as a web app (future will have a standalone app) and build as a debug version

# gather the command line options. Valid are `--web` and `--standalone` for the type of app to build
# and `--release` and `--debug` for the type of build to create

# default values
app_type="web"
build_type="debug"
clean=false

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
        echo "Invalid option: $1"
        exit 1
        ;;
    esac
    shift
done

echo "Building SDRE Hub"

# output the build options

echo "App type: $app_type"
echo "Build type: $build_type"

# clean the build directory
if [ "$clean" = true ]; then
echo "Cleaning the build directory"
cargo clean
fi

# build the app

if [ "$app_type" == "web" ]; then
    echo "Building SDRE Hub as a web app"
    if [ "$build_type" == "debug" ]; then
        echo "Building debug version"
        # build the debug version
        cargo build
    else
        echo "Building release version"
        # build the release version
        cargo build --release
    fi
    else
    echo "Building SDRE Hub as a standalone app"
    echo "Not yet implemented. Exiting"
    if [ "$build_type" == "debug" ]; then
        echo "Building debug version"
        # build the debug version
        # this is a placeholder for the actual build command
    else
        echo "Building release version"
        # build the release version
        # this is a placeholder for the actual build command
    fi
fi
