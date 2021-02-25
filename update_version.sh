#! /bin/bash

CARGO_VERSION=$(cat Cargo.toml | twilight-sparkle --type TOML --expression "package.version")
twilight-sparkle --input package.json --type JSON --expression "version" --replace "\"$CARGO_VERSION\"" --output package.json
npx prettier --write package.json --loglevel silent
