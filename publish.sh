CARGO_VERSION="v$(cat Cargo.toml | twilight-sparkle --type TOML --expression "package.version")"
echo $CARGO_VERSION
git tag $CARGO_VERSION
git push origin $CARGO_VERSION
