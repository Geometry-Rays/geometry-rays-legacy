#!/bin/bash
set -euo pipefail

echo Deleting old builds
cargo clean

echo Building for Linux
cargo build --target x86_64-unknown-linux-gnu --release

echo Building for Windows
cargo build --target x86_64-pc-windows-gnu --release

echo Creating folders
rm -rf Geometry-Rays
mkdir Geometry-Rays

echo Copying compiled executables
cp ./target/x86_64-unknown-linux-gnu/release/geometry-rays ./Geometry-Rays
cp ./target/x86_64-pc-windows-gnu/release/geometry-rays.exe ./Geometry-Rays

echo Copying required folders
cp -r ./Resources ./Geometry-Rays
cp -r ./save-data ./Geometry-Rays
cp -r ./Music ./Geometry-Rays

echo Zipping the package
7z a -tzip Geometry-Rays.zip Geometry-Rays

echo Removing temporary directory
rm -rf ./Geometry-Rays
