#!/usr/bin/env bash

set -e

NEW_VERSION=$1

if [ -z $NEW_VERSION ]
then
    echo "Usage: $0 NEW_VERSION"
    exit 1
fi

perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" Cargo.toml
perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" */Cargo.toml
perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" grammar/*/Cargo.toml
