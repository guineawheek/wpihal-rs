#!/bin/bash
set -eux

DRY_RUN="${DRY_RUN:-'--dry-run'}"

cargo publish -p wpilib-native-utils "$DRY_RUN"
cargo publish -p wpiutil-sys "$DRY_RUN"
cargo publish -p wpiutil "$DRY_RUN"
cargo publish -p wpihal-sys "$DRY_RUN"
cargo publish -p wpihal "$DRY_RUN"